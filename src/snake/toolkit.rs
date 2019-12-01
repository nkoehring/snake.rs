use rand;
use sdl2::pixels::Color;

/// generates a random pastel color
pub fn rand_rgb() -> Color {
    Color::RGB(
        (rand::random::<u8>() / 2) + 127,
        (rand::random::<u8>() / 2) + 127,
        (rand::random::<u8>() / 2) + 127,
    )
}

/// recursively finds a random coordinate that is accepted by the given closure
pub fn rand_coord(
    bounds: &(u32, u32),
    accept: &dyn Fn(&(u32, u32)) -> bool
) -> (u32, u32) {
    let cell = (
        rand::random::<u32>() % bounds.0,
        rand::random::<u32>() % bounds.1,
    );

    if accept(&cell) {
        cell
    } else {
        rand_coord(bounds, accept)
    }
}
