use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub enum Msg {
    ReDraw,
}

#[derive(Properties, PartialEq)]
pub struct PlottersProps {
}

pub struct Plotters {
    canvas: NodeRef,
}

impl Component for Plotters {
    type Message = Msg;
    type Properties = PlottersProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::ReDraw);
        Plotters { canvas: NodeRef::default(), }
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
                drawing_area.fill(&RGBColor(200,200,200)).unwrap();
                      
                let mut chart = ChartBuilder::on(&drawing_area).caption("Test", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(0f32..10f32, 0f32..10f32).unwrap();
                      
                chart.configure_mesh().draw().unwrap();

                chart.draw_series(PointSeries::of_element(
                        vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)],
                        5,
                        ShapeStyle::from(&RED).filled(),
                        &|coord, size, style| {
                            Circle::new(coord, size, style)
                        }
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
                <canvas ref={self.canvas.clone()}/>
            </div>
        }
    }
}
