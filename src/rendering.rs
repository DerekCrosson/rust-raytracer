// Structure for the ray
pub struct Ray {
  pub origin: Point,
  pub direction: Vector3,
}

// Generate prime rays.
impl Ray {
  pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
    // For now assume that the image is wider than it is taller, change
    // this later.
    assert!(scene.width > scene.height);

    // Adjustment for field of view (the angle between the left-most ray
    // and the right-most ray or top and bottom-most). We use simple
    // trigonometry to calculate how much we need to adjust the coordinates by.
    let fov_adjustment = (scene.fov.to_radians() / 2.0).tan()

    // Calculate the aspect ratio and multiply it by the x coordinate.
    // If we don't do this, the rays would be closer together in the x
    // direction than in the y direction, which would cause a distortion 
    // in the image where every pixel would be the same size in both directions.
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);

    let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
    let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

    // Pack the x and y sensor into a vector (z is -1.0 because all prime rays should
    // go forward from the camera) and normalise it to get a nice direction vector.
    // If we’d used any other set of coordinates, the image would be off center and/or
    // we’d have to do additional calculations to avoid distorting it.
    Ray {
      origin: Point::zero(),
      direction: Vector3 {
        x: sensor_x,
        y: sensor_y,
        z: -1.0,
      }
      .normalize(),
    }
  }
}

pub trait Intersectable {
  fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    // Create the sphere.
    let l: Vector3 = self.center - ray.origin;
    let adj = l.dot(&ray.direction);
    let d2 = l.dot(&l) - (adj * adj);
    let radius2 = self.radius * self.radius;
    
    if d2 > radius2 {
     return None;
    }
    
    // Create a triangle between the point that the ray intersects the sphere and the center of the sphere
    let thc = (radius2 - d2).sqrt();
    let t0 = adj - thc;
    let t1 = adj + thc;

    if t0 < 0.0 && t1 < 0.0 {
       return None;
    }
    
    // Use the Pythagoras Theorem to calculate the distance from the right angle to the intersection point.
    let distance = if t0 < t1 { t0 } else { t1 };
    Some(distance)
  }
}