use raylib::prelude::*;

const ROT_SPEED: f32 = 5.0;
const TILEMAP_HEIGHT: usize = 16;
const TILEMAP_WIDTH: usize = 48;
const TILE_WIDTH: usize = 16;

fn allocate_mesh(triangle_count: usize, vertex_count: usize) -> Mesh {
    unsafe {
        let zeroed = std::mem::MaybeUninit::zeroed().assume_init();
        Mesh::from_raw(ffi::Mesh {
            vertexCount: vertex_count as i32,
            triangleCount: triangle_count as i32,
            vertices: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count * 8) as u32).cast(),
            normals: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count * 8) as u32).cast(),
            texcoords: ffi::MemAlloc((size_of::<[f32; 2]>() * vertex_count * 8) as u32).cast(),
            ..zeroed
        })
    }
}

fn gen_block_mesh() -> Mesh {
    let triangle_count = 6 * 2;
    let half = 0.5;
    let mut mesh = allocate_mesh(triangle_count, triangle_count * 3);
    let tilemap_pos_to_uv = |x: usize, y: usize| {
        (
            x as f32 * TILE_WIDTH as f32 / TILEMAP_WIDTH as f32,
            y as f32 * TILE_WIDTH as f32 / TILEMAP_HEIGHT as f32,
        )
    };
    let front_face = (
        [
            [-half, -half, half],
            [half, -half, half],
            [-half, half, half],
            [-half, half, half],
            [half, -half, half],
            [half, half, half],
        ],
        [
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        [[1, 1], [2, 1], [1, 0], [1, 0], [2, 1], [2, 0]],
    );
    let right_face = (
        [
            [half, -half, half],
            [half, -half, -half],
            [half, half, half],
            [half, half, half],
            [half, -half, -half],
            [half, half, -half],
        ],
        [
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ],
        [[1, 1], [2, 1], [1, 0], [1, 0], [2, 1], [2, 0]],
    );
    let back_face = (
        [
            [half, -half, -half],
            [-half, -half, -half],
            [half, half, -half],
            [half, half, -half],
            [-half, -half, -half],
            [-half, half, -half],
        ],
        [
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
        [[1, 1], [2, 1], [1, 0], [1, 0], [2, 1], [2, 0]],
    );
    let left_face = (
        [
            [-half, -half, -half],
            [-half, -half, half],
            [-half, half, -half],
            [-half, half, -half],
            [-half, -half, half],
            [-half, half, half],
        ],
        [
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ],
        [[1, 1], [2, 1], [1, 0], [1, 0], [2, 1], [2, 0]],
    );
    let top_face = (
        [
            [-half, half, -half],
            [-half, half, half],
            [half, half, -half],
            [half, half, -half],
            [-half, half, half],
            [half, half, half],
        ],
        [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        [[2, 1], [2, 0], [3, 1], [3, 1], [2, 0], [3, 0]],
    );
    let bottom_face = (
        [
            [-half, -half, -half],
            [half, -half, -half],
            [-half, -half, half],
            [-half, -half, half],
            [half, -half, -half],
            [half, -half, half],
        ],
        [
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        [[0, 1], [1, 1], [0, 0], [0, 0], [1, 1], [1, 0]],
    );
    let faces = [
        &front_face,
        &back_face,
        &left_face,
        &right_face,
        &bottom_face,
        &top_face,
    ];
    faces
        .iter()
        .flat_map(|(vertices, _, _)| vertices)
        .enumerate()
        .for_each(|(i, v)| {
            let vertex = &mut mesh.vertices_mut()[i];
            vertex.x = v[0];
            vertex.y = v[1];
            vertex.z = v[2];
        });
    faces
        .iter()
        .flat_map(|(_, normals, _)| normals)
        .enumerate()
        .for_each(|(i, n)| {
            let normal = &mut mesh.normals_mut()[i];
            normal.x = n[0];
            normal.y = n[1];
            normal.z = n[2];
        });
    faces
        .iter()
        .flat_map(|(_, _, uvs)| uvs)
        .enumerate()
        .for_each(|(i, u)| {
            let texcoords = &mut mesh.texcoords_mut()[i];
            let (x, y) = tilemap_pos_to_uv(u[0], u[1]);
            texcoords.x = x;
            texcoords.y = y;
        });
    unsafe {
        mesh.upload(false);
    }
    mesh
}

fn update_rot(rot: &mut Vector3, dt: f32) {
    rot.x += ROT_SPEED * dt;
    rot.y += ROT_SPEED * dt / 2.0;
    rot.z += ROT_SPEED * dt / 4.0;
    let pi2 = std::f32::consts::PI * 2.0;
    if rot.x > pi2 {
        rot.x -= pi2;
    }
    if rot.y > pi2 {
        rot.y -= pi2;
    }
    if rot.z > pi2 {
        rot.z -= pi2;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Textured Cube").build();
    let camera = Camera3D::perspective(
        Vector3::new(5.0, 5.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    let cube_tilemap = rl.load_texture(&thread, "static/grass_block.png").unwrap();
    let cube_mesh = gen_block_mesh();
    let mut cube_model = unsafe {
        rl.load_model_from_mesh(&thread, cube_mesh.make_weak())
            .unwrap()
    };
    cube_model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *cube_tilemap.as_ref();
    let mut cube_rot = Vector3::new(0.0, 0.0, 0.0);
    let cube_pos = Vector3::new(0.0, 0.0, 0.0);
    while !rl.window_should_close() {
        update_rot(&mut cube_rot, rl.get_frame_time());
        cube_model.set_transform(&Matrix::rotate_xyz(cube_rot));
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::WHITE);
            d.draw_mode3D(camera, |mut d2| {
                d2.draw_model(&cube_model, cube_pos, 1.0, Color::WHITE);
                d2.draw_model_wires(&cube_model, cube_pos, 1.0, Color::BLACK);
            });
        });
    }
}
