

#[cfg(test)]
mod test {
    use bevy::prelude::*;

    #[derive(Component, Debug, Clone)]
    pub struct Test(u32);

    #[derive(Component, Debug, Clone)]
    pub struct TestStruct {
        field1: u32,
        field2: u8,
        field3: u64,
    };

    #[test]
    fn delta() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);

        app.world.spawn().insert(Test(0)).insert(TestStruct { field1: 0, field2: 0, field3: 0 });
    }
}