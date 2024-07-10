use grs::gr;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Board<const N: usize, const M: usize> {
    data: [[i32; M]; N],
}

fn show<const N: usize, const M: usize>(
    board: &Board<N, M>,
    pred: impl FnMut(&Board<N, M>) -> bool,
) -> Option<()> {
    let path = board.solve(pred);
    println!("number of moves: {}", path.len() - 1);
    gr::setcolormap(1038); // GNUPLOT
    for mut b in path {
        for i in 0..N * M {
            b.data[i / M][i % M] = 12 * b.data[i / M][i % M] + 10;
        }
        gr::cellarray(
            ((0.0, 1.0), (0, 0)),
            ((0.0, 1.0), (M, N)),
            b.data.as_ref().into(),
        )
        .ok()?;
        gr::updatews();
        std::thread::sleep(std::time::Duration::from_millis(600));
    }
    Some(())
}

fn main() {
    let board = [
        [1, 2, 2, 2, 0, 3],
        [1, 0, 4, 5, 0, 3],
        [6, 6, 4, 5, 0, 3],
        [0, 0, 4, 7, 8, 8],
        [0, 9, 9, 7, 10, 0],
        [0, 11, 11, 11, 10, 0],
    ];
    show(&Board { data: board }, |b| {
        b.data[2][4] == 6 && b.data[2][5] == 6
    })
    .expect("everything should work");
}

#[derive(Copy, Clone, Debug)]
struct BoardData<const N: usize, const M: usize> {
    board: Board<N, M>,
    cost: usize,
    prev_hash: Option<u64>,
    done: bool,
}

impl<const N: usize, const M: usize> Board<N, M> {
    fn hash_code(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    fn pieces(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut seen = [[false; M]; N];
        let mut pieces = vec![];
        for y in 0..N {
            for x in 0..M {
                if self.data[y][x] <= 0 || seen[y][x] {
                    continue;
                }
                seen[y][x] = true;
                let mut nx = x;
                while nx < M - 1 && self.data[y][nx + 1] == self.data[y][x] {
                    nx += 1;
                    seen[y][nx] = true;
                }
                let mut ny = y;
                if nx == x {
                    while ny < N - 1 && self.data[ny + 1][x] == self.data[y][x] {
                        ny += 1;
                        seen[ny][x] = true;
                    }
                }
                pieces.push(((y, x), (ny, nx)))
            }
        }
        pieces
    }

    fn swaps(&self, s: (usize, usize), e: (usize, usize)) -> Vec<((usize, usize), (usize, usize))> {
        let mut swaps = vec![];
        if s.0 == e.0 {
            // horizontal
            if e.1 != M - 1 {
                // right
                swaps.push((s, (e.0, e.1 + 1)))
            }
            if s.1 != 0 {
                // left
                swaps.push((e, (s.0, s.1 - 1)))
            }
        }
        if s.1 == e.1 {
            // vertical
            if s.0 != 0 {
                // up
                swaps.push((e, (s.0 - 1, s.1)));
            }
            if e.0 != N - 1 {
                // down
                swaps.push((s, (e.0 + 1, e.1)));
            }
        }
        swaps
    }

    fn solve(&self, mut predicate: impl FnMut(&Board<N, M>) -> bool) -> Vec<Board<N, M>> {
        let bd = BoardData {
            board: *self,
            cost: 0,
            prev_hash: None,
            done: false,
        };
        let mut best: Option<u64> = None;
        let mut queue = vec![self.hash_code()];
        let mut dict = HashMap::new();
        dict.insert(self.hash_code(), bd);
        while let Some((i, &h)) = queue.iter().enumerate().min_by_key(|(_, h)| dict[h].cost) {
            queue.remove(i);
            let bd = dict
                .get_mut(&h)
                .expect("BoardData should be stored when pushing into queue");
            bd.done = true;
            let bd = *bd;
            if predicate(&bd.board) {
                match best.as_ref() {
                    Some(best) if dict[best].cost <= bd.cost => (),
                    _ => {
                        best = Some(bd.board.hash_code());
                    }
                }
                continue;
            }
            let pieces = bd.board.pieces();
            for (p1, p2) in pieces
                .into_iter()
                .flat_map(|(s, e)| bd.board.swaps(s, e))
                .filter(|(_, p2)| bd.board.data[p2.0][p2.1] == 0)
            {
                let mut nb = bd.board.data;
                (nb[p1.0][p1.1], nb[p2.0][p2.1]) = (nb[p2.0][p2.1], nb[p1.0][p1.1]);
                let nb = Board { data: nb };
                let nh = nb.hash_code();
                match dict.get_mut(&nh) {
                    Some(nbd) => {
                        if nbd.cost > bd.cost + 1 {
                            nbd.cost = bd.cost + 1;
                            nbd.prev_hash = Some(h);
                        }
                    }
                    None => {
                        let nbd = BoardData {
                            board: nb,
                            cost: bd.cost + 1,
                            prev_hash: Some(h),
                            done: false,
                        };
                        dict.insert(nh, nbd);
                        queue.push(nh);
                    }
                }
            }
        }
        let mut next = best;
        let mut path = vec![];
        while let Some(curr) = next {
            let bd = dict[&curr];
            path.insert(0, bd.board);
            next = bd.prev_hash;
        }
        path
    }
}
