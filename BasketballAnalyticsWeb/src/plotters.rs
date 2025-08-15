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

                let mut player_points: Vec<(i32,f64)> = Vec::new();
                let mut player_assists: Vec<(i32,f64)> = Vec::new();
                let mut player_rebounds: Vec<(i32, f64)> = Vec::new();
                let mut max_points: f64 = 0.0;
                let mut max_season: i32 = 0;

                if self.data.len() > 0 {
                    max_season = self.data.len() as i32;
                }

                for season in &self.data {
                    if season.points > max_points {
                        max_points = season.points as f64;
                    }
                    player_points.push((season.season_number, season.points));
                    player_assists.push((season.season_number, season.assists));
                    player_rebounds.push((season.season_number, season.rebounds));
                }

                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);
                let backend = CanvasBackend::with_canvas_object(element).unwrap();

                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&WHITE).unwrap();
                      
                let mut chart = ChartBuilder::on(&drawing_area).caption("Test", ("sans-serif", 14).into_font())
                    .margin(10)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(0i32..max_season, 0f64..max_points).unwrap();
                      
                chart.configure_mesh()
                    .disable_mesh()
                    .draw()
                    .unwrap();

                /*
                chart.draw_series(PointSeries::of_element(
                        player_points.clone(),
                        5,
                        ShapeStyle::from(&RED).filled(),
                        &|coord, size, style| {
                            return EmptyElement::at(coord)
                            + Circle::new((0,0), size, style)
                            + Text::new(format!("{:?}", coord.1), (10, 0), ("sans-serif", 10).into_font());
                        },
                )).unwrap()
                    .label("Points Per Season")
                    .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)],&RED));
                */

                // Points Per Game
                chart.draw_series(LineSeries::new(player_points, &RED)).unwrap()
                    .label("Points Per Game")
                    .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)],&RED));

                // Assists Per Game
                chart.draw_series(LineSeries::new(player_assists, &BLUE)).unwrap()
                    .label("Assists Per Game")
                    .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)],&BLUE));

                chart.draw_series(LineSeries::new(player_rebounds, &GREEN)).unwrap()
                    .label("Rebounds Per Game")
                    .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)], &GREEN));

                chart.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.8))
                    .draw()
                    .unwrap();

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
            <div class="chart-container">
                <canvas ref={self.canvas.clone()}/>
            </div>
        }
    }
}
