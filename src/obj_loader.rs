use std::{fmt::Debug, path::Path};

use glium::implement_vertex;

#[derive(Clone, Copy, Debug)]
pub struct ObjVertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(ObjVertex, position, normal);

pub fn load_obj<P>(path: P) -> (Vec<ObjVertex>, Vec<u32>)
where
    P: AsRef<Path> + Debug,
{
    let (models, _) = tobj::load_obj(&path, &tobj::GPU_LOAD_OPTIONS)
        .unwrap_or_else(|_| panic!("Failed to load OBJ file: {:?}", path.as_ref()));

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        println!("Uploading model: {}", model.name);
        assert!(mesh.positions.len() % 3 == 0);
        assert!(mesh.normals.len() % 3 == 0);
        assert!(mesh.indices.len() % 3 == 0);

        let new_indices = mesh
            .indices
            .iter()
            .map(|i| i + vertices.len() as u32)
            .collect::<Vec<_>>();

        let n = mesh.positions.len() / 3;
        if mesh.normals.is_empty() {
            for i in 0..n {
                let position = [
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                ];
                vertices.push(ObjVertex {
                    position,
                    normal: [0.; 3],
                })
            }
        } else {
            assert!(mesh.normals.len() == mesh.positions.len());
            for i in 0..n {
                let position = [
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                ];
                let normal = [
                    mesh.normals[3 * i],
                    mesh.normals[3 * i + 1],
                    mesh.normals[3 * i + 2],
                ];
                vertices.push(ObjVertex { position, normal })
            }
        }

        indices = [indices, new_indices].concat();
    }

    println!(
        "Load OBJ successfully, vertex count: {}, index count: {}",
        vertices.len(),
        indices.len()
    );
    (vertices, indices)
}
