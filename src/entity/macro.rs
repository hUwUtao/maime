#[macro_export]
macro_rules! ent {
    ($pl:ident { $t: item }) => {
        pub struct $pl;
        impl Plugin for $pl {
            fn build(&self, app: &mut App) {
                app.add_systems(Startup, Self::init)
                    .add_systems(Update, Self::tick);
                $t
                init(app);
            }
        }
    };
}
