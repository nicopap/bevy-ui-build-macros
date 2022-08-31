# Bevy UI Build macros

A collection of macros to speed up the process of defining UIs in bevy.

## Bevy version

We do not specify the bevy version we depend on. The reason is mostly that I
rely on a personal fork of bevy, and it's a complete hassle to fork every
bevy library to make them work with my own fork of bevy.

Since this does nothing else than define macros, there is no compiled code in
this crate. the only requirements for the macro to work is that the few bevy
symbols we rely on are in scope where the macros are called. The bevy symbols we
explicity use are:

* `bevy::ecs::system::Insert`
* `bevy::math::{Rect::{self, all}, Size::new}`
* `bevy::prelude::Parent`
* `bevy::ui::{entity::NodeBundle, Style, Val::{Percent, Px}}`

## Macros

The macros are nothing more than wrappers around the struct commonly used when
defining bevy UIs.

```rust
// unit! -- UNIT --
unit!(10 px); unit!(100 pct); 
// Equivalent to
Val::Px(10.0); Val::Percent(100.0);

// style! -- STYLE --
style! {
  flex_wrap: FlexWrap::Wrap,
  flex_basis: unit!(90 pct),
};
// Equivalent to
Style {
   flex_wrap: FlexWrap::Wrap,
   flex_basis: unit!(90 pct),
   ..Default::default()
};

// size! -- SIZE --
size!(100 pct, 90 pct); size!(90 px, 40 px);
// Equivalent to
Size::new(Val::Percent(100.0), Val::Percent(90.0));
Size::new(Val::Px(90.0), Val::Px(40.0));

// rect! -- RECT --
rect!(10 px); rect!(5 px, 0 px); rect!(100 pct, 10 pct, 50 pct, 0 pct);
// Equivalent to
Rect::all(Val::Px(10.0));
Rect {
    left: Val::Px(5.0),
    right: Val::Px(5.0),
    top: Val::Px(0.0),
    bottom: Val::Px(0.0),
};
Rect {
    left: Val::Percent(100.0),
    top: Val::Percent(10.0),
    right: Val::Percent(50.0),
    bottom: Val::Percent(0.0),
};

// build_ui! -- BUILD_UI --
build_ui! {
     #[cmd(commands)]
     vertical{size:size!(100 pct, 100 pct)}(
         horizontal{justify_content: FlexStart, flex_basis: unit!(10 pct)}(
             tab_square[;focus], tab_square[;focus], tab_square[;focus]
         ),
         column_box(
             column[;red](
                 vertical(select_square, select_square),
                 horizontal{flex_wrap: Wrap}[gray](
                     square[;focus], square[;focus], square[;focus], square[;focus],
                     square[;focus], square[;focus], square[;focus], square[;focus],
                     square[;focus], square[;focus], square[;focus], square[;focus]
                 ),
                 horizontal{flex_wrap: Wrap}[gray](
                     square[;focus], square[;focus], square[;focus], square[;focus],
                     square[;focus], square[;focus], square[;focus], square[;focus]
                 )
             )
         )
     )
}
// Equivalent to
commands.spawn_bundle(NodeBundle {
    style: Style { size: size!(100 pct, 100 pct), .. vertical.style },
    .. vertical
})
  .with_children(|cmds| {
    cmds.spawn_bundle(NodeBundle {
        style: Style {justify_content: FlexStart, flex_basis: unit!(10 pct), .. horizontal.style },
        .. horizontal
    })
      .with_children(|cmds| {
        cmds.spawn_bundle(tab_square).insert(focus);
        cmds.spawn_bundle(tab_square).insert(focus);
        cmds.spawn_bundle(tab_square).insert(focus);
      });
    cmds.spawn_bundle(column_box)
      .with_children(|cmds| {
        cmds.spawn_bundle(column).insert(red)
          .with_children(|cmds| {
            vertical.with_children(|cmds| {
              cmds.spawn_bundle(select_square);
              cmds.spawn_bundle(select_square);
            });
            cmds.spawn_bundle(NodeBundle {
                style: Style {flex_wrap: Wrap, ..horizontal.style},
                .. horizontal
            }).insert(gray)
              .with_children(|cmds| {
                for _ in 0..12 {
                  cmds.spawn_bundle(square).insert(focus);
                }
              });
            cmds.spawn_bundle(NodeBundle {
                style: Style {flex_wrap: Wrap, ..horizontal.style},
                .. horizontal
            }).insert(gray)
              .with_children(|cmds| {
                for _ in 0..8 {
                  cmds.spawn_bundle(square).insert(focus);
                }
              });
          });
      });
  });
```

## Changelog

* `0.4.0`: Bevy 0.8 support, use `bevy::ui::Size` and `bevy::ui::UiRect`
  over `math::Size` and `math::Rect`
* `0.3.0`
  * Now it's possible in `build_ui` to end the children list with a comma
  * Added conditional children. Just wrap the children you want to only add
    conditionally in a `if`, you can add as many `if` in a children list. This
    also support `if else`. **Important**: you need to put your predicate between
    parenthesis `if (predicate) { ... }` due to a limitation of rust macros.
* `0.2.1`
  * Added `undefined` and `auto` arguments to `unit!`, this also applies to
    unit-style arguments to `size!` and `rect!`
* `0.2.0`
  * Breaking: The `build_ui!` macro now uses [bundle;comp] instead of just a
    list of components between square brackets. This makes it possible to
    specify a list of components. This however requires having the semicolon
    at the beginning of the square brackets if you only want to specify
    additional components. `[;like, this]`


## License

This is weird. Since this code literally cannot produce distributable binary
code, I don't think any of the mainstream license apply. On top of that, I
usually license small libraries under the WTFPL. But I'll be a good community
player and dual license under MIT or Apache 2.0 at your leisure.

Copyright © Nicola Papale, see LICENSE file for licensing details.
