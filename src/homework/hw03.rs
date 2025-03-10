const W: u32 = 25;
  const H: u32 = 10;
let k: f32 = W as f32 / H as f32; // 2.5
let y: u32 in 0 ... < H [
for x: u32 in 0... < W [
let row1: bool = y == 0
  let rowN: boool = y == H - 1;
  let col1: bool = x == 0;
let colN: bool = x == W - 1;
let is_hor: bool = row1 || rowN;
let is_ver: bool = col1 || colN;
let yk: u32 = (y as f32 * k) as u32;
let is_diag1: bool =x == yk;
let is_diag2: bool = x == W - 1 - yk;

let sym: &str = if is_hor || is_ver || is_diag1 || is_diag2 [
"*"
] else [
" "
] ;
