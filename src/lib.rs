use std::io::BufReader;
use std::fs::File;
use pyo3::prelude::*;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyfunction]
pub fn calculate_volume(filepath: &str) -> PyResult<f32> {
    let mesh = import(filepath);
    let triangles = &mesh.triangles;
    let vertices = &mesh.vertices;
    let mut volume = 0.0;
    
    
    for triangle in triangles {
        let triangle = *triangle as usize;
        let tri_verts = vertices[triangle]; 
        let v1 = tri_verts[0];
        let v2 = tri_verts[1];
        let v3 = tri_verts[2];
        let cross = cross(v1,v2);

        let dot = dot(cross, v3);

        let v = (1.0 / 6.0) * dot;
        volume = volume + v;
    }
    
    Ok(volume)
}

#[pyfunction]
pub fn calculate_surface_area(filepath: &str) -> PyResult<f32> {
    let mesh = import(filepath);
    let triangles = &mesh.triangles;
    let mut area = 0.0;
    
    for triangle in triangles {
        let triangle = *triangle as usize;
        let vertices = mesh.vertices[triangle];
        let tri_area = calculate_triangle_area(vertices);
        area = area + tri_area;
    }
    
    Ok(area)
}

#[pyfunction]
pub fn calculate_centroid(filepath: &str) -> PyResult<Vec<f32>> {
    let mesh = import(filepath);
    let triangles = mesh.triangles;
    let tri_count : f32 = triangles.len() as f32;
    let mut centroid : Vec<f32> = Vec::new();
    let mut cx : f32 = 0.0;
    let mut cy : f32 = 0.0;
    let mut cz : f32 = 0.0;
    
    for triangle in triangles {
        let triangle = triangle as usize;

        let vertices = mesh.vertices[triangle];
    
        let v1 = vertices[0];
        let v2 = vertices[1];
        let v3 = vertices[2];

        let x_avg = (v1[0] + v2[0] + v3[0]) / 3.0;
        let y_avg = (v1[1] + v2[1] + v3[1]) / 3.0;
        let z_avg = (v1[2] + v2[2] + v3[2]) / 3.0;

        cx = cx + x_avg as f32;
        cy = cy + y_avg as f32;
        cz = cz + z_avg as f32;
    }

    cx = cx / tri_count;
    cy = cy / tri_count;
    cz = cz / tri_count;

    centroid.push(cx);
    centroid.push(cy);
    centroid.push(cz);

    Ok(centroid)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_volume, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_centroid, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_surface_area, m)?)?;
    Ok(())
}

// OTHER FUNCTIONS & STRUCTS



pub fn import(path: &str) -> Mesh {
    let path = String::from(path);
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let file = File::open(filename).unwrap();
    let file = File::open(path).unwrap();
    let mut root_vase = BufReader::new(&file);
    let nom_mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
    let mesh = create_mesh(nom_mesh);

    mesh
}

pub fn create_mesh(nom_mesh: nom_stl::Mesh) -> Mesh {

    let nom_tris = nom_mesh.triangles();
    let mut i = 0;
    let mut id_vector: Vec<i32> = Vec::new();
    let mut vertex_vector: Vec<[[f32; 3]; 3]> = Vec::new();

    for triangle in nom_tris {
        let vertices = triangle.vertices();
        id_vector.push(i);
        vertex_vector.push(vertices);
        i = i + 1;
        // println!("{}{:?}",i,vertices)
    }

    let new_mesh = Mesh {
        triangles: id_vector,
        vertices: vertex_vector
    };

    new_mesh    

}

pub struct Mesh {
    pub triangles: Vec<i32>,
    pub vertices: Vec<[[f32; 3]; 3]>,
}

pub fn calculate_triangle_area(triangle: [[f32; 3]; 3]) -> f32 {

    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];

    let cross = cross(ab, ac);

    let area = norm(cross) / 2.0;

    area
}

//Cross product of two 3D vectors
pub fn cross(a : [f32; 3], b : [f32; 3]) -> [f32; 3] {
    
    let i = a[1] * b[2] - a[2] * b[1];
    
    let j = a[0] * b[2] - a[2] * b[0];
    
    let k = a[0] * b[1] - a[1] * b[0];
    
    let cross = [i, -j, k];
    
    cross
    
}

//Dot product of two 3D vectors
pub fn dot(a : [f32; 3], b : [f32; 3]) -> f32 {
    let i = a[0] * b[0];

    let j = a[1] * b[1];

    let k = a[2] * b[2];
    
    let dot = i + j + k;
    
    dot
}

//Magnitude of vector

pub fn norm(v: [f32;3]) -> f32 {

    let a = v[0] as f32;
    let b = v[1] as f32;
    let c = v[2] as f32;

    let a = f32::powi(a,2);
    let b = f32::powi(b,2);
    let c = f32::powi(c,2);

    let sum = a + b + c;

    let norm = sum.sqrt();

    norm

}

pub fn minmax(array: Vec<f32>) -> (f32,f32) {

    let mut min: f32 = 0.0;
    let mut max: f32 = 0.0;

    for item in array {
        if item < min {
            min = item
        } else if item > max {
            max = item
        }
    };

    (min,max)

}