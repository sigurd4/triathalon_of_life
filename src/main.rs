#![windows_subsystem = "windows"]

use std::cmp::Ordering;

use array_init::array_init;
use piston_window::{PistonWindow, WindowSettings, Event, Input, ButtonState, Button, Key, IdleEvent};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const WINDOW_SIZE: [f64; 2] = [640.0, 480.0];

const BAR_WIDTH: f64 = 10.0;
const COLOR_COUNT: usize = 3;
const COLORS: [[f32; 4]; COLOR_COUNT] = [
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    /*[0.0, 1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
    [0.9, 0.9, 0.0, 1.0]*/
];
const AIR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const RES: [usize; 2] = [630/5, 480/5];

const PLACEMENTS: usize = RES[0]*RES[1]*2/9/COLOR_COUNT;

const CLOCK_SKIP: usize = 10;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("triathalon_of_life", WINDOW_SIZE)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
        
    let rng = &mut rand::thread_rng();

    let mut grid: [[Option<usize>; RES[0]]; RES[1]] = new_grid(rng);

    let mut play: bool = false;

    let mut t: usize = 0;
    
    while let Some(event) = window.next()
    {
        match event
        {
            Event::Input(inp, _) => {
                match inp
                {
                    Input::Button(button_args) => {
                        let state = match button_args.state
                        {
                            ButtonState::Press => true,
                            ButtonState::Release => false
                        };
                        match button_args.button
                        {
                            Button::Keyboard(Key::Space) => {
                                if state
                                {
                                    play = !play;
                                }
                            },
                            Button::Keyboard(Key::R) => {
                                if state
                                {
                                    grid = new_grid(rng);
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
            _ => {
                t = (t + 1)%CLOCK_SKIP;
                if play && t == 0
                {
                    let world_move: [isize; 2] = [
                        {
                            let c0 = (1..(RES[1] - 1))
                                .filter(|i| {
                                    let j = 0;
                                    match (grid[*i][j], grid[*i - 1][j], grid[*i + 1][j])
                                    {
                                        (Some(color0), Some(color1), Some(color2)) => color0 == color1 && color0 == color2,
                                        _ => false
                                    }
                                })
                                .count();
                            let c1 = (1..(RES[1] - 1))
                                .filter(|i| {
                                    let j = RES[0] - 1;
                                    match (grid[*i][j], grid[*i - 1][j], grid[*i + 1][j])
                                    {
                                        (Some(color0), Some(color1), Some(color2)) => color0 == color1 && color0 == color2,
                                        _ => false
                                    }
                                })
                                .count();
                            match c0
                                .cmp(&c1)
                            {
                                Ordering::Greater => 1,
                                Ordering::Equal => 0,
                                Ordering::Less => -1
                            }
                        },
                        {
                            let c0 = (1..(RES[0] - 1))
                                .filter(|i| {
                                    let j = 0;
                                    match (grid[j][*i], grid[j][*i - 1], grid[j][*i + 1])
                                    {
                                        (Some(color0), Some(color1), Some(color2)) => color0 == color1 && color0 == color2,
                                        _ => false
                                    }
                                })
                                .count();
                            let c1 = (1..(RES[0] - 1))
                                .filter(|i| {
                                    let j = RES[1] - 1;
                                    match (grid[j][*i], grid[j][*i - 1], grid[j][*i + 1])
                                    {
                                        (Some(color0), Some(color1), Some(color2)) => color0 == color1 && color0 == color2,
                                        _ => false
                                    }
                                })
                                .count();
                            match c0
                                .cmp(&c1)
                            {
                                Ordering::Greater => 1,
                                Ordering::Equal => 0,
                                Ordering::Less => -1
                            }
                        }
                    ];
                    grid = array_init(|i| array_init(|j| {
                        if (i as isize) < world_move[1] || (j as isize) < world_move[0]
                        {
                            None
                        }
                        else
                        {
                            let i: usize = (i as isize - world_move[1]) as usize;
                            let j: usize = (j as isize - world_move[0]) as usize;
                            grid.get(i)
                                .and_then(|collumn|
                                    collumn.get(j)
                                )
                                .and_then(|color| *color)
                        }
                    }));
                    grid = array_init(|i| array_init(|j| {
                        let mut color = grid[i][j];

                        let mut n: [usize; COLOR_COUNT] = [0; COLOR_COUNT];
                        for c in 0..COLOR_COUNT
                        {
                            for di in -1..=1
                            {
                                for dj in -1..=1
                                {
                                    if di != 0 || dj != 0
                                    {
                                        let i0 = i as isize - di;
                                        let j0 = j as isize - dj;
                                        if i0 >= 0 && j0 >= 0 && grid.get(i0 as usize)
                                            .and_then(|gi| gi.get(j0 as usize)
                                                .and_then(|gij| *gij)
                                            ) == Some(c)
                                        {
                                            n[c] += 1;
                                        }
                                    }
                                }
                            }
                        }
                        let n_sum = n.iter().map(|nc| *nc).reduce(|a, b| a + b).unwrap_or(0);
                        match color
                        {
                            Some(c) => if n[c] < 2 || n_sum > 3
                            {
                                color = None
                            },
                            None => if n_sum == 3
                            {
                                let c3: Vec<usize> = n.iter()
                                    .enumerate()
                                    .filter_map(|(c, nc)| if *nc == 3 {Some(c)} else {None})
                                    .collect();
                                if c3.len() == 1
                                {
                                    color = Some(c3[0])
                                }
                                else
                                {
                                    let c2: Vec<usize> = n.iter()
                                        .enumerate()
                                        .filter_map(|(c, nc)| if *nc == 2 {Some(c)} else {None})
                                        .collect();
                                    if c2.len() == 1
                                    {
                                        color = Some(c2[0])
                                    }
                                }
                            }
                        }
                        
                        color
                    }));
                }
                let count: [usize; COLOR_COUNT] = array_init(|c| grid.iter()
                    .map(|gi| gi.iter()
                        .filter(|gij| **gij == Some(c))
                        .count()
                    ).reduce(|a, b| a + b)
                    .unwrap_or(0)
                );
                let count_tot: usize = count.iter()
                    .map(|c| *c)
                    .reduce(|a, b| a + b)
                    .unwrap_or(0);
                let bar: [f64; COLOR_COUNT] = array_init(|c| (count[c] as f64)/(count_tot as f64));
                
                window.draw_2d(&event, |context, graphics, _device| {
                    match context.viewport
                    {
                        Some(viewport) => {
                            graphics::clear(AIR_COLOR, graphics);
                            
                            let window_size = viewport.window_size;
                            let transform = context.transform;

                            let top_bar = bar.iter()
                                .enumerate()
                                .map(|(c, b)| (vec![c], b))
                                .reduce(|(c0, b0), (c1, b1)| if b0 == b1 {([c0, c1].concat(), b0)} else if b0 > b1 {(c0, b0)} else {(c1, b1)})
                                .map(|(c, _)| c)
                                .unwrap();
                            for (i, c) in top_bar.iter().enumerate()
                            {
                                graphics::rectangle(COLORS[*c],
                                    graphics::rectangle::rectangle_by_corners(
                                        (i as f64)/(top_bar.len() as f64)*window_size[0],
                                        0.0,
                                        ((i + 1) as f64)/(top_bar.len() as f64)*window_size[0],
                                        BAR_WIDTH/2.0
                                    ),
                                    transform,
                                    graphics
                                );
                            }
                            let mut bar_acc: f64 = 0.0;
                            for c in 0..COLOR_COUNT
                            {
                                graphics::rectangle(COLORS[c],
                                    graphics::rectangle::rectangle_by_corners(
                                        bar_acc*window_size[0],
                                        BAR_WIDTH/2.0,
                                        (bar_acc + bar[c])*window_size[0],
                                        BAR_WIDTH
                                    ),
                                    transform,
                                    graphics
                                );
                                bar_acc += bar[c]
                            }
                            
                            for i in 0..RES[1]
                            {
                                for j in 0..RES[0]
                                {
                                    match grid[i][j]
                                    {
                                        Some(c) => graphics::rectangle(COLORS[c],
                                            graphics::rectangle::rectangle_by_corners(
                                                (j as f64)/(RES[0] as f64)*window_size[0],
                                                (i as f64)/(RES[1] as f64)*(window_size[1] - BAR_WIDTH) + BAR_WIDTH,
                                                ((j + 1) as f64)/(RES[0] as f64)*window_size[0],
                                                ((i + 1) as f64)/(RES[1] as f64)*(window_size[1] - BAR_WIDTH) + BAR_WIDTH
                                            ),
                                            transform,
                                            graphics
                                        ),
                                        _ => ()
                                    };
                                }
                            }
                        },
                        _ => ()
                    }
                });
            }
        }
    }
}

fn new_grid(rng: &mut ThreadRng) -> [[Option<usize>; RES[0]]; RES[1]]
{
    let mut grid: [[Option<usize>; RES[0]]; RES[1]] = [[None; RES[0]]; RES[1]];

    let mut positions: [[usize; 2]; RES[0]*RES[1]] = array_init(|i| [i%RES[0], i/RES[0]]);
    positions.shuffle(rng);
    
    let mut i = 0;

    let mut colors: [usize; COLOR_COUNT] = array_init(|i| i);

    for _ in 0..PLACEMENTS
    {
        colors.shuffle(rng);
        
        for color in colors
        {
            grid[positions[i][1]][positions[i][0]] = Some(color);
            i += 1;
        }
    }

    grid
}