use rocketsim::sim::arena::Arena;

#[test]
fn iter_pads() {
    rocketsim::init();

    let arena = Arena::default_soccar();
    assert!(arena.iter_pads_state().collect<Vec<_>>().len() == arena.num_pads());
}

#[test]
fn iter_cars() {
    rocketsim::init();

    let arena = Arena::default_soccar();
    assert!(arena.iter_cars_state().collect<Vec<_>>().len() == arena.num_cars());
}
