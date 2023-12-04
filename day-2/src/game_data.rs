pub mod game_data {
    use crate::cube_set::cube_set::CubeSet;

    #[derive(Debug)]
    pub struct GameData {
        pub(crate) game_num: u32,
        pub(crate) rounds: Vec<CubeSet>,
    }
}