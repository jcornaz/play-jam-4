#![cfg(feature = "std")]

use rstest::rstest;

use collision::Aabb;

#[rstest]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([2., 0.], [3., 1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([-2., 0.], [-1., 1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., -2.], [0., -1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., 2.], [0., 3.]))]
fn returns_none_if_does_not_collide(#[case] shape1: Aabb, #[case] shape2: Aabb) {
    assert_eq!(shape1.penetration(shape2), None);
}

#[test]
fn should_penetrate_self() {
    let shape = Aabb::from_min_max([0., 0.], [1., 1.]);
    let [x, y] = shape
        .penetration(shape)
        .expect("shape does not penetrate itself");
    assert_eq!(x.abs() + y.abs(), 1.);
}

#[rstest]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0.5, 0.], [1.5, 1.]), [-0.5, 0.0])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([-0.5, 0.], [0.5, 1.]), [0.5, 0.0])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., -0.5], [1.0, 0.5]), [0.0, 0.5])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., 0.5], [1.0, 1.5]), [0.0, -0.5])]
fn returns_expected_penetration_when_shapes_collide(
    #[case] shape1: Aabb,
    #[case] shape2: Aabb,
    #[case] expected_penetration: [f32; 2],
) {
    println!("shape1: {shape1:?}");
    println!("shape2: {shape2:?}");
    let actual_penetration = shape1
        .penetration(shape2)
        .expect("the shape are not penetrating");
    assert!(
        (actual_penetration[0] - expected_penetration[0]).abs() <= f32::EPSILON,
        "{actual_penetration:?} != {expected_penetration:?}"
    );
    assert!(
        (actual_penetration[1] - expected_penetration[1]).abs() <= f32::EPSILON,
        "{actual_penetration:?} != {expected_penetration:?}"
    );
}
