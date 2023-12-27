use std::array;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("lavaduct lagoon area: {}", lavaduct_lagoon_area(INPUT));
}

struct Point {
    y: isize,
    x: isize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Point")
            .field(&self.y)
            .field(&self.x)
            .finish()
    }
}

impl Point {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

fn lavaduct_lagoon_area(input: &str) -> usize {
    let mut circumference = 0;
    let mut polygon: Vec<Point> = Vec::new();
    {
        let mut x: isize = 0;
        let mut y: isize = 0;

        for line in input.lines() {
            let mut line = line.split_whitespace();
            let [dir, stride, _color] = array::from_fn(|_| line.next().unwrap());
            let stride = stride.parse::<usize>().unwrap();

            circumference += stride;

            match dir {
                "R" => x += stride as isize,
                "L" => x -= stride as isize,
                "D" => y += stride as isize,
                "U" => y -= stride as isize,
                _ => panic!("{dir:?}"),
            }

            polygon.push(Point::new(y, x));
        }
    }

    let mut area = 0isize;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        area += (polygon[j].x + polygon[i].x) * (polygon[j].y - polygon[i].y);
        j = i;
    }
    (area / 2).unsigned_abs() + (circumference / 2) + 1
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat! {
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)",
    };

    #[test]
    fn lavaduct_lagoon() {
        let result = super::lavaduct_lagoon_area(INPUT);
        assert_eq!(result, 62);
    }
}
