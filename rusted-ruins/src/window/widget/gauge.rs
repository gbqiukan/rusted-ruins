
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use super::{WidgetTrait, LabelWidget};
use sdlvalues::{SdlValues, FontKind};
use config::UI_CFG;

/// Bar gauge widget.
pub struct GaugeWidget {
    rect: Rect,
    colors: Colors,
    value: f32, min: f32, max: f32,
    label: Option<LabelWidget>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GaugeColorMode {
    Hp,
}

impl GaugeColorMode {
    fn colors(&self) -> Colors {
        match self {
            GaugeColorMode::Hp => Colors {
                bar: UI_CFG.color.gauge_hp.into(),
                bg: UI_CFG.color.gauge_bg.into(),
                border_light: UI_CFG.color.border_light.into(),
                border_dark: UI_CFG.color.border_dark.into(),
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Colors {
    bg: Color,
    bar: Color,
    border_light: Color,
    border_dark: Color,
}

impl GaugeWidget {
    pub fn new(rect: Rect, min: f32, max: f32, mode: GaugeColorMode) -> GaugeWidget {
        GaugeWidget {
            rect,
            colors: mode.colors(),
            label: None,
            value: min, min, max,
        }
    }

    pub fn with_label(rect: Rect, min: f32, max: f32, mode: GaugeColorMode, text: &str) -> GaugeWidget {
        GaugeWidget {
            rect,
            colors: mode.colors(),
            label: Some(LabelWidget::bordered(rect, text, FontKind::MonoM)),
            value: min, min, max,
        }
    }

    pub fn set_params(&mut self, min: f32, max: f32, value: f32) {
        self.value = value;
        self.max = max;
        self.min = min;
    }
}

impl WidgetTrait for GaugeWidget {
    type Response =  ();

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {

        canvas.set_draw_color(self.colors.bg);
        check_draw!(canvas.fill_rect(self.rect));

        let value = if self.value >= self.min { self.value } else { self.min };
        let bar_width =
            ((self.rect.w - 4) as f32 * ((value - self.min) / (self.max - self.min))) as u32;
        let bar_rect = Rect::new(2, 2, bar_width, self.rect.height() - 2);

        canvas.set_draw_color(self.colors.bar);
        check_draw!(canvas.fill_rect(bar_rect));
        
        for n in 0..2 {
            let r = Rect::new(
                self.rect.x + n, self.rect.y + n,
                (self.rect.w - 2 * n) as u32, (self.rect.h - 2 * n) as u32);
            let c: Color = if n == 0 { self.colors.border_dark } else { self.colors.border_light };

            canvas.set_draw_color(c);
            check_draw!(canvas.draw_rect(r));
        }

        if let Some(ref mut label) = self.label {
            label.draw(canvas, sv);
        }
    }
}

