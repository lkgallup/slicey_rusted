use nalgebra;
use std::env;
use std::fs::OpenOptions;
use std::path;
use stl_io::{
    self, 
    IndexedTriangle, 
    Vector
};

pub type Edge = [Vector<f32>; 2];
type Face = IndexedTriangle;
type Triangle = [Vector<f32>; 3];
type Vertex = Vector<f32>;

pub type Edges = Vec<Edge>;
type Faces = Vec<Face>;
type Triangles = Vec<Triangle>;
type Vertices = Vec<Vertex>;

pub trait RayIntersection {
    fn ray_intersection(
        &self, 
        point: nalgebra::Point3::<f32>,
        ray: nalgebra::Vector3::<f32>
    ) -> () {

    }
}

impl RayIntersection for Triangle {}

///
#[derive(Debug)]
pub struct BoundingBox {
    pub x_min: f32,
    pub y_min: f32,
    pub z_min: f32,
    pub x_max: f32,
    pub y_max: f32,
    pub z_max: f32
}

///
#[derive(Clone, Debug)]
pub struct STLMesh {
    faces: Faces,
    file_name: String,
    triangles: Triangles,
    vertices: Vertices
}

impl STLMesh {
    pub fn new(file_name: String) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&file_name)
            // .unwrap();
            .expect(
                format!(
                    "Failed to open STL file. Path is {:?} and current dir is {:?}", 
                    path::Path::new(&file_name),
                    env::current_dir().unwrap()
                ).as_str()
            );
        let stl = stl_io::read_stl(&mut file).unwrap();

        // let triangles = STLMesh::_triangles(&stl);
        let triangles = STLMesh::_triangles(&stl.faces, &stl.vertices);
        STLMesh {
            faces: stl.faces,
            file_name: file_name,
            triangles: triangles,
            vertices: stl.vertices
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let x_min = self.vertices()
            .iter()
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let x_max = self.vertices()
            .iter()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        BoundingBox {
            x_min: x_min[0], 
            y_min: x_min[1],
            z_min: x_min[2],
            x_max: x_max[0],
            y_max: x_max[1],
            z_max: x_max[2]
        }
    }

    pub fn faces(&self) -> &Vec<IndexedTriangle> {
        &self.faces
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn home_z(&mut self) -> () {
        let bb = self.bounding_box();
        self.translate(0., 0., -bb.z_min);
    }

    pub fn is_inside(&self, point: nalgebra::Point3::<f32>) -> bool {
        // arbitrary ray in +x direction
        let ray = nalgebra::Vector3::<f32>::new(1., 0., 0.);
        
        let mut count = 0
        for tri in self.triangles() {
            if tri.ray_intersection(point, &ray) {
                count = count + 1;
            }
        }

        count % 2 == 1
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) -> () {
        self.vertices = self.vertices
            .iter_mut()
            .map(|a| Vector::new(
                [x * a[0], y * a[1], z * a[2]]
            ))
            .collect::<Vec<Vector<f32>>>();
        self.triangles = STLMesh::_triangles(&self.faces, &self.vertices);
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> () {
        self.vertices = self.vertices
            .iter_mut()
            .map(|a| Vector::new(
                [a[0] + x, a[1] + y, a[2] + z]
            ))
            .collect::<Vec<Vector<f32>>>();
        self.triangles = STLMesh::_triangles(&self.faces, &self.vertices);
    }

    // pub fn _triangles(stl: &IndexedMesh) -> Vec<[Vector<f32>; 3]> {
    /// helper method for generating triangles
    /// should only really be called once for each STL
    /// unless of course you scale, translate, etc.
    pub fn _triangles(faces: &Faces, vertices: &Vertices) -> Triangles {
        let tris: Vec<_> = faces
            .iter()
            .map(|x| [
                vertices[x.vertices[0]], 
                vertices[x.vertices[1]], 
                vertices[x.vertices[2]]
            ])
            .collect();
        tris
    } 

    pub fn triangles(&self) -> &Vec<[Vector<f32>; 3]> {
        &self.triangles
    }

    pub fn vertices(&self) -> &Vec<Vector<f32>> {
        &self.vertices
    }

    // pub fn write_stl(&self, file_name: &str) -> () {
    //     let mut file = OpenOptions::new()
    //         .write(true)
    //         .create_new(true)
    //         .open("mesh.stl")
    //         .unwrap();
    //     stl_io::write_stl(&mut file, self.stl.iter()).unwrap();
    // }
}
