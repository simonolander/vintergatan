use std::collections::HashSet;

struct Universe {
    width: usize,
    height: usize,
    neighbours: HashSet<(usize, usize)>
}