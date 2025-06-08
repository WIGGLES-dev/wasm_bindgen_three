use js_sys::{Array, Function, Object};
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlCanvasElement, HtmlElement};

#[wasm_bindgen]
#[wasm_bindgen(getter_with_clone)]
pub struct WebGlRendererParameters {
    pub canvas: Option<HtmlCanvasElement>,
    pub antialias: bool,
}

#[wasm_bindgen]
#[wasm_bindgen(getter_with_clone)]
pub struct MeshBasicMaterialParameters {
    pub color: u32,
}

impl MeshBasicMaterial {
    pub fn new(params: MeshBasicMaterialParameters) -> Self {
        Self::constructor(JsValue::from(params).unchecked_ref())
    }
    pub fn set_color(&self, color: u32) {
        set_material_color(&self, color);
    }
}

impl WebGLRenderer {
    pub fn new(params: WebGlRendererParameters) -> Self {
        Self::constructor(JsValue::from(params).unchecked_ref())
    }
}

impl Object3D {
    pub fn set_position(&self, x: f32, y: f32, z: f32) {
        batch_update_position(self, x, y, z);
    }
    pub fn set_scale(&self, x: f32, y: f32, z: f32) {
        batch_update_scale(self, x, y, z);
    }
    pub fn set_quaternion(&self, x: f32, y: f32, z: f32, w: f32) {
        batch_update_quaternion(self, x, y, z, w);
    }
    pub fn set_transform(
        &self,
        x: f32,
        y: f32,
        z: f32,
        sx: f32,
        sy: f32,
        sz: f32,
        qx: f32,
        qy: f32,
        qz: f32,
        qw: f32,
    ) {
        batch_update_transform(self, x, y, z, sx, sy, sz, qx, qy, qz, qw);
    }
    pub fn add_position(&self, x: f32, y: f32, z: f32) {
        batch_add_position(self, x, y, z);
    }
}

impl Raycaster {
    pub fn ndc(
        canvas_x: f32,
        canvas_y: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> [f32; 2] {
        let x = (canvas_x / viewport_width) * 2. - 1.;
        let y = -(canvas_y / viewport_height) * 2. + 1.;
        [x, y]
    }

    pub fn set_from_camera_and_ndc(&self, camera: &Camera, [ndc_x, ndc_y]: [f32; 2]) {
        set_raycaster_from_camera_and_ndc(self, camera, ndc_x, ndc_y);
    }
}

#[derive(Default)]
pub enum LoopMode {
    Once = 2200,
    #[default]
    Repeat = 2201,
    PingPong = 2202,
}

#[wasm_bindgen(raw_module = "three")]
extern "C" {
    #[derive(Clone)]
    pub type WebGLRenderer;
    #[wasm_bindgen(constructor)]
    pub fn constructor(parameters: &Object) -> WebGLRenderer;
    #[wasm_bindgen(constructor)]
    pub fn default() -> WebGLRenderer;

    #[wasm_bindgen(method)]
    pub fn render(this: &WebGLRenderer, scene: &Object3D, camera: &Camera);
    #[wasm_bindgen(method, js_name = "setAnimationLoop")]
    pub fn set_animation_loop(this: &WebGLRenderer, animate: &Function);
    #[wasm_bindgen(method, js_name = "setSize")]
    pub fn set_size(this: &WebGLRenderer, width: f32, height: f32, update_style: bool);
    #[wasm_bindgen(method, getter, js_name = "domElement")]
    pub fn dom_element(this: &WebGLRenderer) -> HtmlCanvasElement;
    #[wasm_bindgen(method, js_name = "setClearColor")]
    pub fn set_clear_color(this: &WebGLRenderer, color: u32);

    #[derive(Clone)]
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Object3D;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Object3D) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn uuid(this: &Object3D) -> String;
    #[wasm_bindgen(method, variadic)]
    pub fn add(this: &Object3D, children: Vec<Object3D>);
    #[wasm_bindgen(method, js_name = "add")]
    pub fn add_1(this: &Object3D, child: &Object3D);
    #[wasm_bindgen(method, js_name = "removeFromParent")]
    pub fn remove_from_parent(this: &Object3D);
    #[wasm_bindgen(method, variadic)]
    pub fn remove(this: &Object3D, children: Array);
    #[wasm_bindgen(method)]
    pub fn remove_1(this: &Object3D, child: &Object3D);
    #[wasm_bindgen(method, getter)]
    pub fn layers(this: &Object3D) -> Layers;
    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Object3D) -> Array;
    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Object3D) -> Vector3;
    #[wasm_bindgen(method, js_name = "updateMatrix")]
    pub fn update_matrix(this: &Object3D);
    #[wasm_bindgen(method, js_name = "updateMatrixWorld")]
    pub fn update_matrix_world(this: &Object3D, force: bool);

    #[derive(Clone)]
    #[wasm_bindgen(extends = Object3D)]
    pub type Scene;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;

    #[wasm_bindgen(extends = Object3D)]
    pub type Group;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Group;

    #[derive(Clone)]
    #[wasm_bindgen(extends = Object3D)]
    pub type Camera;

    #[derive(Clone)]
    #[wasm_bindgen(extends = Camera)]
    pub type PerspectiveCamera;
    #[wasm_bindgen(constructor)]
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> PerspectiveCamera;
    #[wasm_bindgen(method, js_name = "updateProjectionMatrix")]
    pub fn update_projection_matrix(this: &PerspectiveCamera);
    #[wasm_bindgen(method, getter)]
    pub fn aspect(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_aspect(this: &PerspectiveCamera, aspect: f32);
    #[wasm_bindgen(method, getter)]
    pub fn far(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_far(this: &PerspectiveCamera, far: f32);
    #[wasm_bindgen(method, getter)]
    pub fn film_gauge(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_film_gauge(this: &PerspectiveCamera, film_guage: f32);
    #[wasm_bindgen(method, getter)]
    pub fn film_offset(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_film_offset(this: &PerspectiveCamera, film_offset: f32);
    #[wasm_bindgen(method, getter)]
    pub fn focus(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_focus(this: &PerspectiveCamera, focus: f32);
    #[wasm_bindgen(method, getter)]
    pub fn fov(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_fov(this: &PerspectiveCamera, fov: f32);
    #[wasm_bindgen(method, getter)]
    pub fn near(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_near(this: &PerspectiveCamera, near: f32);
    #[wasm_bindgen(method, getter)]
    pub fn view(this: &PerspectiveCamera) -> js_sys::Object;
    #[wasm_bindgen(method, setter)]
    pub fn set_view(this: &PerspectiveCamera, view: &js_sys::Object);
    #[wasm_bindgen(method, getter)]
    pub fn zoom(this: &PerspectiveCamera) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_zoom(this: &PerspectiveCamera, zoom: f32);

    #[wasm_bindgen(extends = Object3D)]
    pub type Mesh;
    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &BufferGeometry, material: &Material) -> Mesh;
    #[wasm_bindgen(constructor)]
    pub fn default() -> Mesh;
    #[wasm_bindgen(method, setter)]
    pub fn set_geometry(this: &Mesh, geometry: &BufferGeometry);
    #[wasm_bindgen(method, setter)]
    pub fn set_material(this: &Mesh, material: &Material);
    #[wasm_bindgen(method, setter = material)]
    pub fn set_material_list(this: &Mesh, material: Array);

    #[derive(Clone)]
    pub type BufferGeometry;
    #[wasm_bindgen(method)]
    pub fn dispose(this: &BufferGeometry);

    #[derive(Clone)]
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type BoxGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(w: f32, l: f32, h: f32) -> BoxGeometry;

    #[derive(Clone)]
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type CircleGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f32, segments: u32, theta_start: f32, theta_length: f32) -> CircleGeometry;

    #[derive(Clone)]
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type CylinderGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        radial_segments: u32,
        height_segments: u32,
        open_ended: bool,
        theta_start: f32,
        theta_length: f32,
    ) -> CylinderGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_radius(radius_top: f32, radius_bottom: f32, height: f32) -> CylinderGeometry;

    #[derive(Clone)]
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type SphereGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(
        radius: f32,
        width_segments: u32,
        height_segments: f32,
        phi_start: f32,
        phi_length: f32,
        theta_start: f32,
        theta_length: f32,
    ) -> SphereGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_radius(radius: f32) -> SphereGeometry;

    #[wasm_bindgen(extends = BufferGeometry)]
    pub type PlaneGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32) -> PlaneGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_with_segments(
        width: f32,
        height: f32,
        width_segments: u32,
        height_segments: u32,
    ) -> PlaneGeometry;

    #[derive(Clone)]
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type ShapeGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(shape: &Shape) -> ShapeGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_many(shapes: &Array) -> ShapeGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_with_segments(shape: &Shape, curve_segments: u32) -> ShapeGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new_many_with_segments(shapes: &Array, cureve_segments: u32) -> ShapeGeometry;

    pub type Curve;

    #[wasm_bindgen(extends = Curve)]
    pub type CurvePath;

    #[wasm_bindgen(extends = CurvePath)]
    pub type Path;

    #[wasm_bindgen(extends = Path)]
    pub type Shape;
    pub fn new(points: Option<Array>) -> Shape;
    #[wasm_bindgen(method, js_name = "moveTo")]
    pub fn move_to(this: &Shape);
    #[wasm_bindgen(method, js_name = "lineTo")]
    pub fn line_to(this: &Shape);
    #[wasm_bindgen(method, js_name = "closePath")]
    pub fn close_path(this: &Shape);

    #[derive(Clone)]
    pub type Material;

    #[derive(Clone)]
    #[wasm_bindgen(extends = Material)]
    pub type MeshBasicMaterial;
    #[wasm_bindgen(constructor)]
    pub fn constructor(parameters: &Object) -> MeshBasicMaterial;
    #[wasm_bindgen(constructor)]
    pub fn default() -> MeshBasicMaterial;

    #[wasm_bindgen(extends = Material)]
    pub type MeshStandardMaterial;
    #[wasm_bindgen(constructor)]
    pub fn constructor(parameters: &Object) -> MeshStandardMaterial;
    #[wasm_bindgen(constructor)]
    pub fn default() -> MeshStandardMaterial;

    pub type Vector2;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Vector2;
    #[wasm_bindgen(constructor)]
    pub fn default() -> Vector2;

    pub type Vector3;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Vector3;
    #[wasm_bindgen(constructor)]
    pub fn default() -> Vector3;

    pub type Euler;
    pub type Mat4;

    pub type Plane;

    #[derive(Clone)]
    pub type Raycaster;
    #[wasm_bindgen(constructor)]
    pub fn constructor(origin: &Vector3, direction: &Vector3, near: f32, far: f32) -> Raycaster;
    #[wasm_bindgen(constructor)]
    pub fn default() -> Raycaster;
    #[wasm_bindgen(method, js_name = "intersectObjects")]
    pub fn intersect_objects(
        this: &Raycaster,
        objects: Array,
        recursive: bool,
        optional_target: Option<Array>,
    ) -> Array;
    #[wasm_bindgen(method, js_name = "intersectObject")]
    pub fn intersect_object(
        this: &Raycaster,
        object: &Object,
        recursive: bool,
        optional_target: Array,
    ) -> Array;
    #[wasm_bindgen(method, js_name = "setFromCamera")]
    pub fn set_from_camera(this: &Raycaster, pointer: &Vector2, camera: &Camera);
    #[wasm_bindgen(method, getter)]
    pub fn layers(this: &Raycaster) -> Layers;

    #[wasm_bindgen(method, getter)]
    pub fn ray(this: &Raycaster) -> Ray;

    pub type Ray;

    pub type MathUtils;

    #[wasm_bindgen]
    pub type Layers;
    #[wasm_bindgen(method, getter)]
    pub fn mask(this: &Layers) -> u32;
    #[wasm_bindgen(method)]
    pub fn disable(this: &Layers, layer: u8);
    #[wasm_bindgen(method)]
    pub fn enable(this: &Layers, layer: u8);
    #[wasm_bindgen(method)]
    pub fn set(this: &Layers, layer: u8);
    #[wasm_bindgen(method)]
    pub fn test(this: &Layers, layers: &Layers) -> bool;
    #[wasm_bindgen(method, js_name = "isEnabled")]
    pub fn is_enabled(this: &Layers, layers: &Layers) -> bool;
    #[wasm_bindgen(method)]
    pub fn toggle(this: &Layers, layer: u8);
    #[wasm_bindgen(method)]
    pub fn enable_all(this: &Layers);
    #[wasm_bindgen(method)]
    pub fn disbaled_all(this: &Layers);

    #[derive(Clone)]
    pub type EventDispatcher;
    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventDispatcher, event: &str, f: &Function);
    #[wasm_bindgen(method, js_name = "removeEventListener")]
    pub fn remove_event_listner(this: &EventDispatcher, event: &str, f: &Function);

    pub type AnimationAction;
    #[wasm_bindgen(constructor)]
    pub fn new(
        mixer: &AnimationMixer,
        clip: &AnimationClip,
        local_root: &Object3D,
    ) -> AnimationAction;
    #[wasm_bindgen(method, getter = blendMode)]
    pub fn blend_mode(this: &AnimationAction) -> u32;
    #[wasm_bindgen(method, setter = blendMode)]
    pub fn set_blend_mode(this: &AnimationAction) -> u32;

    pub type AnimationClip;
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, duration: usize, tracks: Array) -> AnimationClip;

    pub type AnimationMixer;
    #[wasm_bindgen(constructor)]
    pub fn new(root_object: &Object3D) -> AnimationMixer;
    #[wasm_bindgen(method, getter)]
    pub fn time(this: &AnimationMixer) -> usize;
    #[wasm_bindgen(method, getter)]
    pub fn time_scale(this: &AnimationMixer) -> usize;
    #[wasm_bindgen(method, setter)]
    pub fn set_time_scale(this: &AnimationMixer, time_scale: usize);
    #[wasm_bindgen(method, js_name = "clipAction")]
    pub fn clip_action(
        this: &AnimationMixer,
        clip: &AnimationClip,
        optional_root: &Object3D,
    ) -> AnimationAction;
    #[wasm_bindgen(method, js_name = "existingAction")]
    pub fn existing_action(
        this: &AnimationMixer,
        clip: &AnimationClip,
        optional_root: &Object3D,
    ) -> AnimationAction;
    #[wasm_bindgen(method, js_name = "getRoot")]
    pub fn get_root(this: &AnimationMixer) -> Object3D;
    #[wasm_bindgen(method, js_name = "stopAllAction")]
    pub fn stop_all_action(this: &AnimationMixer) -> AnimationMixer;
    #[wasm_bindgen(method)]
    pub fn update(this: &AnimationMixer) -> AnimationMixer;
    #[wasm_bindgen(method)]
    pub fn set_time(this: &AnimationMixer, time_in_seconds: usize) -> AnimationMixer;
    #[wasm_bindgen(method, js_name = "uncacheClip")]
    pub fn uncache_clip(this: &AnimationMixer, clip: &AnimationClip);
    #[wasm_bindgen(method, js_name = "uncacheRoot")]
    pub fn uncache_root(this: &AnimationMixer, root: &Object3D);
    #[wasm_bindgen(method, js_name = "uncacheAction")]
    pub fn uncache_action(this: &AnimationMixer, clip: &AnimationClip, optional_root: &Object3D);

    pub type AnimationObjectGroup;
    #[wasm_bindgen(constructor)]
    pub fn default() -> AnimationObjectGroup;
    #[wasm_bindgen(constructor, variadic)]
    pub fn new(objects: Vec<Object3D>) -> AnimationObjectGroup;
    #[wasm_bindgen(method, getter)]
    pub fn stats(this: &AnimationObjectGroup) -> Object;
    #[wasm_bindgen(method, getter)]
    pub fn uuid(this: &AnimationObjectGroup) -> String;
    #[wasm_bindgen(method, variadic)]
    pub fn add(this: &AnimationObjectGroup, objects: Vec<Object3D>);
    #[wasm_bindgen(method, js_name = "add")]
    pub fn add_1(this: &AnimationObjectGroup, object: &Object3D);
    #[wasm_bindgen(method, variadic)]
    pub fn remove(this: &AnimationObjectGroup, objects: Vec<Object3D>);
    #[wasm_bindgen(method, js_name = "remove")]
    pub fn remove_1(this: &AnimationObjectGroup, objects: &Object3D);
    #[wasm_bindgen(method, variadic)]
    pub fn uncache(this: &AnimationObjectGroup, objects: Vec<Object3D>);
    #[wasm_bindgen(method, js_name = "remove")]
    pub fn uncache_1(this: &AnimationObjectGroup, object: &Object3D);

    #[derive(Clone)]
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Controls;
    #[wasm_bindgen(constructor)]
    pub fn new(object: &Object3D, dom_element: Option<&HtmlElement>) -> Controls;
    #[wasm_bindgen(method, getter = domElement)]
    pub fn dom_element(this: &Controls) -> HtmlElement;
    #[wasm_bindgen(method, getter)]
    pub fn enabled(this: &Controls) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_enabled(this: &Controls, enabled: bool);
    #[wasm_bindgen(method)]
    pub fn connect(this: &Controls, element: &HtmlElement);
    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Controls);
    #[wasm_bindgen(method)]
    pub fn dispose(this: &Controls);
    #[wasm_bindgen(method)]
    pub fn update(this: &Controls, delta: usize);
    #[wasm_bindgen(method, getter)]
    pub fn object(this: &Controls) -> Object3D;
    #[wasm_bindgen(method, setter)]
    pub fn set_object(this: &Controls, object: &Object3D);

    pub type Texture;

    pub type LoadingManager;

    #[derive(Clone)]
    pub type Loader;
    #[derive(Clone)]
    #[wasm_bindgen(extends = Loader)]
    pub type TextureLoader;
    #[wasm_bindgen(constructor)]
    pub fn new() -> TextureLoader;
    #[wasm_bindgen(constructor)]
    pub fn new_with_manager(manager: &LoadingManager) -> TextureLoader;
    #[wasm_bindgen(method)]
    pub fn load(this: &TextureLoader, url: &str) -> Texture;
    #[wasm_bindgen(method, js_name = "load")]
    pub fn load_with_callbacks(
        this: &TextureLoader,
        url: &str,
        on_loader: &Function,
        on_progress: &Function,
        on_error: &Function,
    ) -> Texture;
}

#[wasm_bindgen(raw_module = "three/addons/controls/DragControls.js")]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(extends = Controls)]
    pub type DragControls;
    #[wasm_bindgen(constructor)]
    pub fn new(objects: &Array, camera: &Camera, dom_element: Option<&HtmlElement>)
    -> DragControls;
}

#[wasm_bindgen(raw_module = "three/addons/renderers/Css3DRenderer.js")]
extern "C" {
    #[derive(Clone)]
    pub type CSS3DRenderer;
    #[wasm_bindgen(constructor)]
    pub fn default() -> CSS3DRenderer;

    #[derive(Clone)]
    #[wasm_bindgen(extends = Object3D)]
    pub type CSS3DObject;
    #[wasm_bindgen(constructor)]
    pub fn new(element: HtmlElement) -> CSS3DObject;
    #[wasm_bindgen(constructor)]
    pub fn default() -> CSS3DObject;

    #[wasm_bindgen(method, getter)]
    pub fn get_element(this: &CSS3DObject) -> HtmlElement;
    #[wasm_bindgen(method, setter)]
    pub fn set_element(this: &CSS3DObject, element: &HtmlElement) -> HtmlElement;

    #[derive(Clone)]
    pub type CSS3DSprite;
    #[wasm_bindgen(constructor)]
    pub fn new(element: HtmlElement) -> CSS3DSprite;
}

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    pub fn set_user_data(object: &Object3D, key: &str, v: &JsValue);

    pub fn get_user_data(object: &Object3D, key: &str) -> JsValue;

    pub fn batch_update_scale(object: &Object3D, x: f32, y: f32, z: f32);
    pub fn batch_update_quaternion(object: &Object3D, x: f32, y: f32, z: f32, w: f32);
    pub fn batch_update_position(object: &Object3D, x: f32, y: f32, z: f32);
    pub fn batch_update_rotation(object: &Object3D, x: f32, y: f32, z: f32);

    pub fn batch_add_position(object: &Object3D, x: f32, y: f32, z: f32);

    pub fn batch_update_transform(
        object: &Object3D,
        x: f32,
        y: f32,
        z: f32,
        sx: f32,
        sy: f32,
        sz: f32,
        qx: f32,
        qy: f32,
        qz: f32,
        qw: f32,
    );

    #[wasm_bindgen(variadic)]
    pub fn add_materials_to_object(object: &Object3D, materials: Array);

    pub fn set_object_material(object: &Object3D, material: &Material);
    #[wasm_bindgen(js_name = "set_object_material", variadic)]
    pub fn set_object_material_variadic(object: &Object3D, material: Array);

    pub fn reparent_object(object: &Object3D, parent: &Object3D);

    pub fn set_raycaster_from_camera_and_ndc(raycater: &Raycaster, camera: &Camera, x: f32, y: f32);

    pub fn lerp_object_to(object: &Object3D, tx: f32, ty: f32, tz: f32, alpha: f32);

    pub fn set_material_color(material: &MeshBasicMaterial, color: u32);

    pub fn intersect_xy_plane(raycaster: &Raycaster) -> Box<[f32]>;

    pub fn create_path_from_points(points: &[f32]) -> Path;
    pub fn create_shape_from_points(points: &[f32]) -> Shape;

    pub fn get_screen_position(
        object: &Object3D,
        camera: &Camera,
        renderer: &WebGLRenderer,
    ) -> Vec<u32>;

    pub fn set_raycaster_from_camera_renderer_xy(
        raycaster: &Raycaster,
        camera: &Camera,
        renderer: &WebGLRenderer,
        screen_x: f32,
        screen_y: f32,
    );

    pub type Intersections;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Intersections;
}
