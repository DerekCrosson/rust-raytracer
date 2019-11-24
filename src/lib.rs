pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
  let black = Rgba::from_channels(0, 0, 0, 0);
  for x in 0..scene.width {
    for y in 0..scene.height {
      let ray = Ray::create_prime(x, y, scene);

      if scene.sphere.intersect(&ray) {
        image.put_pixel(x, y, to_rgba(&scene.sphere.color))
      } else {
        image.put_pixel(x, y, black);
      }
    }
  }
  image
}


fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
  // Calculate surface normal of the object at the point the ray intersected with it.
  let hit_point = ray.origin + (ray.direction * intersection.distance);
  // Add an albedo to the Spheres and Planes (a parameter which specifies how much
  // light energy is reflected by an object and how much is absorbed). Then implement
  // shading.
  
  let surface_normal = intersection.element.surface_normal(&hit_point);
  let direction_to_light = -scene.light.direction.normalize();
  
  // Calculate the amount of light that lands on this point. This is proportional to
  // the cosine of the angle between the surface normal and the direction to the
  // light (Lambert’s Cosine Law). The dot product is the length of one vector multiplied
  // by the cosine of the angle between them, but because normalised vectors are used the
  // length will be one. Also added a factor for the brightness of the light.
  let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                  scene.light.intensity;
  
  // Calculate the proportion of the light which is reflected. This is equal to the albedo
  // of the object divided by Pi. Dividing by Pi ensures that the object doesn’t reflect
  // away more energy than it receives.
  let light_reflected = intersection.element.albedo() / std::f32::consts::PI;

  // Represent colors as (R, G, B) triplets where each value is in the range 0.0…1.0.
  let color = intersection.element.color().clone() * scene.light.color.clone() * light_power *
            light_reflected;
  color.clamp()
}

#[test]
fn test_can_render_scene() {
  let scene = Scene {
    width: 800,
    height: 600,
    fov: 90.0,
    sphere: Sphere {
      center: Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
      },
      radius: 1.0,
      color: Color {
        red: 0.4,
        green: 1.0,
        blue: 0.4,
      },
    },
  };

  let img: DynamicImage = render(&scene);
  assert_eq!(scene.width, img.width());
  assert_eq!(scene.height, img.height());
}