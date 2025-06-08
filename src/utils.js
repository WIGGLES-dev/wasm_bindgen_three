import { Vector2, Vector3, Plane, Path, Shape } from "three";

const screenSize = new Vector2();
const screenPosition = new Vector3();
const xy_plane = new Plane(new Vector3(0, 0, 1), 0);
const intersection_cache = new Vector3();
const coords = new Vector2();

export function set_user_data(object, key, value) {
  object.userData[key] = value;
}

export function get_user_data(object, key) {
  return object.userData[key];
}

export function batch_add_position(object, x, y, z) {
  object.position.x += x;
  object.position.y += y;
  object.position.z += z;
}

export function batch_update_quaternion(object, x, y, z, w) {
  object.quaternion.set(x, y, z, w);
}
export function batch_update_position(object, x, y, z) {
  object.position.set(x, y, z);
}

export function batch_update_rotation(object, x, y, z) {
  object.rotation.set(x, y, z);
}

export function batch_update_scale(object, x, y, z) {
  object.scale.set(x, y, z);
}

export function batch_update_transform(
  object,
  x,
  y,
  z,
  sx,
  sy,
  sz,
  qx,
  qy,
  qz,
  qw
) {
  object.position.set(x, y, z);
  object.scale.set(sx, sy, sz);
  object.quaternion.set(qx, qy, qz, qw);
}

export function add_materials_to_object(object, ...materials) {
  object.materials.push(...materials);
}

export function set_object_material(object, material) {
  object.material = material;
}

export function set_object_geometry(object, geometry) {
  object.geometry = geometry;
}

export function reparent_object(object, newParent) {
  object.removeFromParent();
  newParent.add(object);
}

export function set_raycaster_from_camera_and_ndc(raycaster, camera, x, y) {
  coords.x = x;
  coords.y = y;
  raycaster.setFromCamera(coords, camera);
}

export function intersect_xy_plane(raycaster) {
  raycaster.ray.intersectPlane(xy_plane, intersection_cache);
  return new Float32Array([
    intersection_cache.x,
    intersection_cache.y,
    intersection_cache.z,
  ]);
}

export function set_material_color(material, color) {
  material.color.set(color);
}

export function lerp_object_to(object, tx, ty, tz, alpha) {
  object.position.lerp({ x: tx, y: ty, z: ty }, alpha);
}

function forward_event(event) {
  event.object.dispatchEvent(event);
}

function set_path_points(path, points) {
  for (let i = 0; i < points.length; i += 2) {
    const x = points[i];
    const y = points[i + 1];

    if (i === 0) {
      path.moveTo(x, y);
    } else {
      path.lineTo(x, y);
    }
  }
  path.closePath();
}
export function create_path_from_points(points) {
  const path = new Path();
  set_path_points(path, points);
  return path;
}
export function create_shape_from_points(points) {
  const shape = new Shape();
  set_path_points(shape, points);
  return shape;
}

export function get_screen_position(object, camera, renderer) {
  object.getWorldPosition(screenPosition);
  screenPosition.project(camera);
  renderer.getSize(screenSize);
  const x = ((screenPosition.x + 1) / 2) * screenSize.x;
  const y = ((-screenPosition.y + 1) / 2) * screenSize.y;
  const array = new Uint32Array([x, y]);
  return array;
}

export function set_raycaster_from_camera_renderer_xy(
  raycaster,
  camera,
  renderer,
  screen_x,
  screen_y
) {
  renderer.getSize(screenSize);
  const x = (screen_x / screenSize.x) * 2 - 1;
  const y = -(screen_y / screenSize.y) * 2 + 1;
  coords.set(x, y);
  raycaster.setFromCamera(coords, camera);
}

export class Intersections {
  constructor(raycaster, object) {}

  static object_from_hits(hits) {
    return hits.map((hit) => hit.object);
  }
}
