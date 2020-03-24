use std::collections::HashMap;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    slug: String,
    background_color: String,
    color: String,
}

#[derive(Debug, Clone)]
pub struct RowTile {
    slug: String,
    row_index: u32,
    tile_index: u32,
    tile: Tile,
}

pub struct BoardRow {
    tile_rows: Vec<RowTile>,
}

pub struct Board {
    rows: Vec<BoardRow>,
}

struct State {
    tiles: HashMap<String, Tile>,
    blank_tile: Tile,
    board: Board,
    dragging: bool,
    selected: Option<Tile>,
}

pub struct Model {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Msg {
    SelectLegendTile(Tile),
    SelectBoardTile(RowTile),
    MaybeSelectBoardTile(RowTile),
    EndDrag,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let blank_tile = Tile::new("white", "red");
        let default_tiles = vec![
            Tile::new("#444", "white"),
            Tile::new("blue", "white"),
            Tile::new("cyan", "blue"),
            Tile::new("pink", "white"),
            Tile::new("yellow", "red"),
            Tile::new("#64c7cc", "cyan"),
            Tile::new("#00a64d", "#75f0c3"),
            Tile::new("#f5008b", "#ffdbbf"),
            Tile::new("#0469bd", "#75d2fa"),
            Tile::new("#fcf000", "#d60000"),
            Tile::new("#010103", "#fa8e66"),
            Tile::new("#7a2c02", "#fff3e6"),
            Tile::new("#f5989c", "#963e03"),
            Tile::new("#ed1c23", "#fff780"),
            Tile::new("#f7f7f7", "#009e4c"),
            Tile::new("#e04696", "#9c2c4b"),
        ];

        let mut tiles = HashMap::new();

        for tile in default_tiles {
            tiles.insert(tile.slug.clone(), tile);
        }

        let state = State {
            tiles: tiles,
            board: Board::new(&blank_tile),
            blank_tile: blank_tile,
            selected: None,
            dragging: false,
        };

        Model { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EndDrag => self.state.dragging = false,
            Msg::SelectLegendTile(tile) => self.state.selected = Some(tile),
            Msg::MaybeSelectBoardTile(row_tile) => {
                if self.state.dragging {
                    let selected_tile = match &self.state.selected {
                        Some(tile) => tile,
                        None => &self.state.blank_tile,
                    };
                    self.state.dragging = true;
                    self.state.board.rows[row_tile.row_index as usize].tile_rows
                        [row_tile.tile_index as usize]
                        .tile = selected_tile.clone()
                }
            }
            Msg::SelectBoardTile(row_tile) => {
                let selected_tile = match &self.state.selected {
                    Some(tile) => tile,
                    None => &self.state.blank_tile,
                };
                self.state.dragging = true;
                self.state.board.rows[row_tile.row_index as usize].tile_rows
                    [row_tile.tile_index as usize]
                    .tile = selected_tile.clone()
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_legend() }
                { self.render_board() }
            </div>
        }
    }
}

impl Model {
    fn render_legend(&self) -> Html {
        let tiles = self.state.get_legend_tiles();
        html! {
            <div class="Legend-wrap">
                { for tiles.iter().map(|tile| self.render_legend_tile(tile)) }
                { self.render_legend_tile(&self.state.blank_tile) }
            </div>
        }
    }

    fn render_legend_tile(&self, tile: &Tile) -> Html {
        let t = tile.clone();
        let mut class_string = "Tile-wrap".to_string();
        if tile.slug == self.state.get_selected_tile().slug {
            class_string = format!("{} {}", class_string, "Tile-selected");
        }
        html! {
            <div class={class_string}
                 onclick=self.link.callback(move |_| Msg::SelectLegendTile(t.clone()))
                 style={format!("background-color: {};", tile.background_color)}>
                <div class="Tile-inner"
                     draggable="false"
                     style={format!("background-color: {};", tile.color)}></div>
            </div>
        }
    }

    fn render_tile(&self, row_tile: &RowTile) -> Html {
        let r = row_tile.clone();
        let b = row_tile.clone();
        html! {
            <div class="Tile-wrap"
                 draggable="false"
                 onmousedown=self.link.callback(move |_| Msg::SelectBoardTile(r.clone()))
                 onmouseup=self.link.callback(move |_| Msg::EndDrag)
                 onmouseover=self.link.callback(move |_| Msg::MaybeSelectBoardTile(b.clone()))
                 style={format!("background-color: {};", row_tile.tile.background_color)}>
                <div class="Tile-inner" style={format!("background-color: {};", row_tile.tile.color)}></div>
            </div>
        }
    }

    fn render_board(&self) -> Html {
        html! {
            <div class="Board-wrap">
                { for self.state.board.rows.iter().map(|row| self.render_row(&row)) }
            </div>
        }
    }

    fn render_row(&self, row: &BoardRow) -> Html {
        html! {
            <div class="Row-wrap">
                { for row.tile_rows.iter().map(|tile| self.render_tile(&tile)) }
            </div>
        }
    }
}

impl State {
    fn get_legend_tiles(&self) -> Vec<Tile> {
        self.tiles.values().cloned().collect::<Vec<Tile>>()
    }

    fn get_selected_tile(&self) -> &Tile {
        match &self.selected {
            Some(tile) => tile,
            None => &self.blank_tile,
        }
    }
}

impl Board {
    fn new(default_tile: &Tile) -> Board {
        let mut rows = Vec::new();
        for n in 0..14 {
            rows.push(BoardRow::new(n, default_tile))
        }
        Board { rows: rows }
    }
}

impl BoardRow {
    fn new(index: u32, default_tile: &Tile) -> BoardRow {
        let t = default_tile.clone();
        let mut tile_rows = Vec::new();
        for n in 0..18 {
            let slug = format!("row-{}-tile-{}", index, n);
            tile_rows.push(RowTile {
                slug: slug,
                row_index: index,
                tile_index: n,
                tile: t.clone(),
            });
        }
        BoardRow {
            tile_rows: tile_rows,
        }
    }
}

impl Tile {
    /// Creates a new Tile.
    fn new(background_color: &str, color: &str) -> Tile {
        let bg_string = background_color.to_string();
        let color_string = color.to_string();
        let slug = format!("{}-{}", bg_string, color_string);

        Tile {
            slug: slug,
            background_color: bg_string,
            color: color_string,
        }
    }
}
