// Structures to hold scene data.

pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
}

pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub elements: Vec<Element>,
  pub light: Light,
}

pub struct Intersection<'a> {
  pub distance: f64,
  pub object: &'a Sphere,
}

pub struct Plane {
  pub p0: Point,
  pub normal: Vector3,
  pub color: Color,
}

pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}

pub struct Light {
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32,
}

impl<'a> Intersection<'a> {
  pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
      // Elided
  }
}

impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self.spheres
      .iter()
      .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}

impl Element {
  pub fn color(&self) -> &Color {
    match *self {
      Element::Sphere(ref s) => &s.color,
      Element::Plane(ref p) => &p.color,
    }
  }
}

impl Intersectable for Element {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    match *self {
      Element::Sphere(ref s) => s.intersect(ray),
      Element::Plane(ref p) => p.intersect(ray),
    }
  }
}

impl Intersectable for Plane {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    let normal = &self.normal;
    let denom = normal.dot(&ray.direction);
    if denom > 1e-6 {
      let v = self.origin - ray.origin;
      let distance = v.dot(&normal) / denom;
      if distance >= 0.0 {
        return Some(distance);
      }
    }
    None
  }
}