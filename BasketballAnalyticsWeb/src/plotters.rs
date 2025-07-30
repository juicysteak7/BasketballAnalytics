use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use crate::{ PlayerSeason };

pub enum Msg {
    ReDraw,
}

#[derive(Properties, PartialEq)]
pub struct PlottersProps {
    pub data: Vec<PlayerSeason>,
}

pub struct Plotters {
    canvas: NodeRef,
    data: Vec<PlayerSeason>,
}

impl Component for Plotters {
    type Message = Msg;
    type Properties = PlottersProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::ReDraw);
        Plotters { canvas: NodeRef::default(), data: ctx.props().data.clone() }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.data = ctx.props().data.clone();
        ctx.link().send_message(Msg::ReDraw);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReDraw => {
                let element: HtmlCanvasElement = self.canvas.cast().unwrap();

                let rect = element.get_bounding_client_rect();
                log::info!("rect width: {:?}, rect height: {:?}", rect.width(), rect.height());
                //element.set_height(rect.height() as u32);
                //element.set_width(rect.width() as u32);

                element.set_height(300);
                element.set_width(600);
                let backend = CanvasBackend::with_canvas_object(element).unwrap();

                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&WHITE).unwrap();
                      
                let mut chart = ChartBuilder::on(&drawing_area).caption("Test", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(0i32..10i32, 0f64..10f64).unwrap();
                      
                chart.configure_mesh().draw().unwrap();

                let mut player_points: Vec<(i32,f64)> = Vec::new();

                for season in &self.data {
                    player_points.push((season.season_number, season.points));
                }

                chart.draw_series(PointSeries::of_element(
                        player_points,
                        5,
                        ShapeStyle::from(&RED).filled(),
                        &|coord, size, style| {
                            return EmptyElement::at(coord)
                            + Circle::new((0,0), size, style)
                            + Text::new(format!("{:?}", coord), (10, 0), ("sans-serif", 10).into_font());
                        },
                )).unwrap();

                /*
                chart.draw_series(LineSeries::new((-50..=50)
                        .map(|x| x as f32 / 50.0).map(|x| (x, x * x)),&RED,)).unwrap()
                        .label("y = x^2")
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                */

                false
            }
            _ => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <canvas style="width:60%;" ref={self.canvas.clone()}/>
            </div>
        }
    }
}
