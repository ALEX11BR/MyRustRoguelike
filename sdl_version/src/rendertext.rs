use std::error::Error;

use common::point::Point;
use sdl2::{pixels::Color, ttf::Font, render::{Canvas, TextureCreator, RenderTarget}, video::WindowContext, rect::Rect};

use crate::{TILE_WIDTH, TILE_HEIGHT};

pub fn render_char<T: RenderTarget>(
    char: char,
    fg_color: Color,
    bg_color: Color,
    point: Point,
    font: &Font,
    canvas: &mut Canvas<T>,
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<(), Box<dyn Error>> {
    let draw_rect = Rect::new(
        point.x * TILE_WIDTH,
        point.y * TILE_HEIGHT,
        TILE_WIDTH as u32,
        TILE_HEIGHT as u32,
    );
    canvas.set_draw_color(bg_color);
    canvas.fill_rect(draw_rect)?;

    let char_surface = font.render_char(char).blended(fg_color)?;
    let char_rect = char_surface.rect();
    let char_texture = texture_creator.create_texture_from_surface(char_surface)?;
    canvas.copy(&char_texture, char_rect, draw_rect)?;

    Ok(())
}

pub fn render_text<T: RenderTarget>(
    text: &str,
    color: Color,
    draw_window: &mut Rect,
    font: &Font,
    canvas: &mut Canvas<T>,
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<(), Box<dyn Error>> {
    let text_surface = font
        .render(text)
        .blended_wrapped(color, draw_window.width())?;
    let text_rect = text_surface.rect();
    let draw_rect = Rect::new(
        draw_window.left(),
        draw_window.top(),
        text_rect.width(),
        text_rect.height(),
    );
    let text_texture = texture_creator.create_texture_from_surface(text_surface)?;
    canvas.copy(&text_texture, text_rect, draw_rect)?;

    draw_window.set_y(draw_window.y() + draw_rect.height() as i32);
    draw_window.set_height(draw_window.height() - draw_rect.height());

    Ok(())
}