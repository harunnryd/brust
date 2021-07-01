fn main() {
    GameState::new();
}

/// Next we create an enum that will represent all the possible
/// directions that our entity could move.
#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Now we define a struct that will hold an entity's position on our game board
/// or grid which we defined above. We'll use signed integers because we only want
/// to store whole numbers, and we need them to be signed so that they work properly
/// with our modulus arithmetic later.
#[derive(Debug, Copy, Clone)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition {x, y}
    }

    /// We'll make another helper function that takes one grid position and returns a new one after
    /// making one move in the direction of `dir`. 
    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, pos.y - 1),
            Direction::Down => GridPosition::new(pos.x, pos.y + 1),
            Direction::Left => GridPosition::new(pos.x - 1, pos.y),
            Direction::Right => GridPosition::new(pos.x + 1, pos.y),
        }
    }
}

impl Direction {
    /// We also create a helper function that will let use convet between a
    /// keycode and the `Direction` that it represents. Of course, not every
    /// keycode represents a direction, so we return `None` if this
    /// is the case.
    pub fn from_keycode(key: &str) -> Option<Direction> {
        match key {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            _ => None,
        }
    }
}


#[derive(Debug)]
struct Spaceship {
    pos: GridPosition,
    direction: Direction,
}

impl Spaceship {
    pub fn new(pos: GridPosition) -> Self {
        Spaceship {pos, direction: Direction::Right}
    }

    fn update(&mut self) {
        // First we get a new position by using our `new_from_move` helper
        // function from earlier. We move our entity in the direction we are currently
        // heading.
        let new_pos = GridPosition::new_from_move(self.pos, self.direction);

        // And finally make our actual entity the new `GridPosition` we created. This has
        // effectively moved the entity in the current direction.
        self.pos = new_pos;
    }
}

#[derive(Debug)]

struct Enemy {
    pos: GridPosition,
    direction: Direction,
}

impl Enemy {
    pub fn new(pos: GridPosition) -> Self {
        Enemy {pos, direction: Direction::Right}
    }

    fn update(&mut self) {
        // First we get a new position by using our `new_from_move` helper
        // function from earlier. We move our entity in the direction we are currently
        // heading.
        let new_pos = GridPosition::new_from_move(self.pos, self.direction);

        // And finally make our actual entity the new `GridPosition` we created. This has
        // effectively moved the entity in the current direction.
        self.pos = new_pos;
    }
}


#[derive(Debug)]

struct Bullet {
    pos: GridPosition,
    direction: Direction,
}

impl Bullet {
    pub fn new(pos: GridPosition) -> Self {
        Bullet {pos, direction: Direction::Right}
    }

    fn update(&mut self) {
        // First we get a new position by using our `new_from_move` helper
        // function from earlier. We move our entity in the direction we are currently
        // heading.
        let new_pos = GridPosition::new_from_move(self.pos, self.direction);

        // And finally make our actual entity the new `GridPosition` we created. This has
        // effectively moved the entity in the current direction.
        self.pos = new_pos;
    }
}

/// Now we have the heart of our game, the GameState.
struct GameState {
    /// First we need a Spaceship
    space_ship: Spaceship,
    /// A piece of Enemy
    enemy: Enemy,
    /// A piece of Enemy
    bullet: Bullet,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            space_ship: Spaceship::new(GridPosition::new(0, 1)),
            enemy: Enemy::new(GridPosition::new(0, 1)),
            bullet: Bullet::new(GridPosition::new(0, 1)),
        }
    }
}