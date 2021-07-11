#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn distance_to(&self, point: &Point) -> f32 {
        //returns distance between self and second point
        (((self.x - point.x).powi(2)) + ((self.y - point.y).powi(2))).sqrt()
    }
    fn in_circumcenter(&self, triangle: &Triangle) -> bool {
        if self.distance_to(&triangle.circumcenter) <= triangle.radius {
            return true;
        }
        false
    }
    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Triangle {
    vertices: [Point; 3],
    edges: [[Point; 2]; 3],
    circumcenter: Point,
    radius: f32,
}
impl Triangle {
    //TODO: Add rendering of Triangles, either render each triangle, or collect all edges and render those
    fn print(&self) {
        let vertices = self.vertices;
        println!(
            "A: ({}, {}) B: ({}, {}) C: ({}, {}) Cent: ({}, {}) Radius: {}",
            vertices[0].x,
            vertices[0].y,
            vertices[1].x,
            vertices[1].y,
            vertices[2].x,
            vertices[2].y,
            self.circumcenter.x,
            self.circumcenter.y,
            self.radius
        )
    }
}

fn build_triangle(points: [Point; 3]) -> Triangle {
    //uses gross equations I found on wikipedia to init a triangle struct with vertecies, circumcenter, and radius of circumscribed circle
    let d = 2.0 //helper variable for circumcenter calc
        * (points[0].x * (points[1].y - points[2].y)
            + points[1].x * (points[2].y - points[0].y)
            + points[2].x * (points[0].y - points[1].y));
    let circumcenter = Point {
        //finds the circumcenter location given triangle points
        x: ((points[0].x.powi(2) + points[0].y.powi(2)) * (points[1].y - points[2].y)
            + (points[1].x.powi(2) + points[1].y.powi(2)) * (points[2].y - points[0].y)
            + (points[2].x.powi(2) + points[2].y.powi(2)) * (points[0].y - points[1].y))
            / d,
        y: ((points[0].x.powi(2) + points[0].y.powi(2)) * (points[2].x - points[1].x)
            + (points[1].x.powi(2) + points[1].y.powi(2)) * (points[0].x - points[2].x)
            + (points[2].x.powi(2) + points[2].y.powi(2)) * (points[1].x - points[0].x))
            / d,
    };
    let radius = points[0].distance_to(&circumcenter);
    let edges = {
        let a = points[0];
        let b = points[1];
        let c = points[2];
        [[a, b], [b, c], [c, a]]
    };

    Triangle {
        vertices: points,
        edges: edges,
        circumcenter: circumcenter,
        radius: radius,
    }
}
fn bowyer_watson_triangulation(points: Vec<Point>) -> Vec<Triangle> {
    //TODO: maybe change this algorithm in a way that allows adding of points after generation, probably a function for the points for loop, possibly add a tag for super triangle connected triangles so they arent outputed/ rendered
    let mut triangulation: Vec<Triangle> = vec![build_triangle([
        Point {
            x: -100000.0,
            y: -100000.0,
        },
        Point {
            x: 100000.0,
            y: 0.0,
        },
        Point {
            x: -100000.0,
            y: 100000.0,
        },
    ])]; //inits super triangle, all points must be inside

    for point in points {
        //loops over each point in cloud
        let mut badtriangles: Vec<Triangle> = Vec::new();
        let mut outer_polygon: Vec<[Point; 2]> = Vec::new();
        let mut full_polygon: Vec<[Point; 2]> = Vec::new();
        for triangle in &triangulation {
            //checks each existing triangle
            if point.in_circumcenter(triangle) {
                //deletes and reconstructs triangles that have conflicting circumcenters
                badtriangles.push(*triangle);
                let edges = triangle.edges;
                for i in 0..=2 {
                    let alt = [edges[i][1], edges[i][0]];
                    if full_polygon.contains(&edges[i]) || full_polygon.contains(&alt) {
                        outer_polygon.retain(|&x| x != edges[i] && x != alt);
                    } else {
                        outer_polygon.push(edges[i]);
                        full_polygon.push(edges[i]);
                    }
                }
            }
        }

        for badtriangle in badtriangles {
            triangulation.retain(|&x| x != badtriangle);
        }

        for edge in outer_polygon {
            triangulation.push(build_triangle([edge[0], edge[1], point]))
        }
    }
    let mut faketriangles: Vec<Triangle> = Vec::new();
    for triangle in &triangulation {
        //removes super triangle connected triangles
        if triangle.vertices.contains(&Point {
            x: -100000.0,
            y: -100000.0,
        }) || triangle.vertices.contains(&Point {
            x: 100000.0,
            y: 0.0,
        }) || triangle.vertices.contains(&Point {
            x: -100000.0,
            y: 100000.0,
        }) {
            faketriangles.push(*triangle)
        }
    }
    for faketriangle in faketriangles {
        triangulation.retain(|&x| x != faketriangle);
    }
    triangulation
}

fn main() {
    let a = Point { x: 10.0, y: 12.0 };
    let b = Point { x: 8.0, y: -1.0 };
    let c = Point { x: 4.0, y: 2.0 };
    let d = Point { x: 10.0, y: 6.0 };
    let e = Point { x: 0.0, y: 0.0 };
    let f = Point { x: 15.0, y: -3.0 };

    let trangle = build_triangle([a, b, c]);

    let points = vec![a, b, c, d, e, f];

    let tranglulate = bowyer_watson_triangulation(points);

    println!("Triangles: {}", tranglulate.len());

    for triangle in tranglulate {
        triangle.print();
    }
}
