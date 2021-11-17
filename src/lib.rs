/// Wrapper around the `bevy::ui::Val` enum
///
/// # Syntax
/// * `unit!(num1 px)` ⇒ `Val::Px(num1 as f32)`
/// * `unit!(num1 pct)` ⇒ `Val::Percent(num1 as f32)`
#[macro_export]
macro_rules! unit {
    (@with_value px $value:literal) => ( bevy::ui::Val::Px($value as f32));
    (@with_value pct $value:literal) => ( bevy::ui::Val::Percent($value as f32));
    ($value:literal $val_unit:ident) => ( unit!(@with_value $val_unit $value));
}

/// Wrapper around `bevy::ui::Style`
///
/// ```rust,ignore
/// style! {
///     param1: something,
///     param2: something_else,
/// }
/// // Is strictly equivalent to
/// bevy::ui::Style {
///     param1: something,
///     param2: something_else,
///     ..Default.default()
/// }
/// ```
#[macro_export]
macro_rules! style {
    (@default ($default:expr) $($field:ident : $content:expr),*) => (
        bevy::ui::Style { $($field : $content,)* .. $default }
    );
    ($($field:ident : $content:expr,)*) => (
        style!(@default (Default::default()) $($field : $content),*)
    );
}

/// Wrapper around `bevy::math::Size::new`
///
/// # Syntax
/// * `size!(num1 val1, num2 val2)` ⇒ `Size::new(unit!(num1 val1), unit!(num2 val2))`
#[macro_export]
macro_rules! size {
    ($x:literal $x_unit:ident, $y:literal $y_unit:ident) => (
        bevy::math::Size::new(unit!($x $x_unit), unit!($y $y_unit))
    );
}

/// Define a `bevy::math::Rect` similarly to how you would define it in CSS.
///
/// # Syntax
/// ```rust,ignore
/// // one argument
/// rect!(num1 val1) == Rect::all(unit!(num1 val1))
///
/// // two arguments
/// rect!(num1 val1, num2 val2) == Rect {
///     left: unit!(num1 val1),
///     right: unit!(num1 val1),
///     top: unit!(num2 val2),
///     bottom: unit!(num2 val2),
/// }
///
/// // four arguments
/// rect!(num1 val1, num2 val2, num3 val3, num4 val4) == Rect {
///     left: unit!(num1 val1),
///     top: unit!(num2 val2),
///     right: unit!(num3 val3),
///     bottom: unit!(num4 val4),
/// }
/// ```
#[macro_export]
macro_rules! rect {
    ($x:literal $x_unit:ident) => (
        bevy::math::Rect::all(unit!($x $x_unit))
    );
    (
        $left:literal $left_unit:ident, $top:literal $top_unit:ident,
        $right:literal $right_unit:ident, $bottom:literal $bottom_unit:ident,
    ) => (
        bevy::math::Rect {
            left: unit!($left $left_unit),
            top: unit!($top $top_unit),
            right: unit!($right $right_unit),
            bottom: unit!($bottom $bottom_unit),
        }
    );
    ($x:literal $x_unit:ident, $y:literal $y_unit:ident) => (
        bevy::math::Rect {
            left: unit!($x $x_unit),
            top: unit!($y $y_unit),
            right: unit!($x $x_unit),
            bottom: unit!($y $y_unit),
        }
    );
}

/// Define a bevy UI and spawns it using `cmd`
///
/// # Syntax
/// ```rust,ignore 
/// use bevy::prelude::*;
/// let commands: Commands;
/// let my_id: Entity;
/// build_ui! {
///     // The bevy `Commands`
///     #[cmd(commands)]
///     // The "preset" is an identifier, see doc
///     $entity
///         // Style modifiers. Supposing $entity is a `NodeBundle`, does:
///         // $entity.style = style!{ flex_whatever: Whatever }
///         // Leads to a compilation error if $entity doesn't have a `style`
///         // field
///         { flex_whatever: Whatever }
///         // Additional components and bundles. Translates to
///         // $entity.insert_bundle(bundle1).insert_bundle(bundle2).insert(comp1).insert(comp2)
///         // If you don't care for bundles or comp, just leave the left or
///         // right of the ; blank
///         [bundle1, bundl2 ;comp1, comp2] 
///         // Children entities, may have {..}, [..;..] and (..) 
///         (
///             entity[ButtonBundle](square),
///             id(my_id)
///         )
/// }
/// ```
///
/// The `$entity` in the macro may be one of the following:
/// * `id(Entity)`: inserts a pre-existing entity as child of containing entity
/// * `$ident`: where `$ident` is the name of a local variable of type 
///   `T: ComponentBundle`. Spawn the bundle as base to insert extra components
///   to. Useful to not repeat yourself.
/// * `entity`: spawn an empty bundle as base to insert extra components to.
///
/// # Example
///
/// ```rust,ignore
/// build_ui! {
///     #[cmd(commands)]
///     vertical{size:size!(100 pct, 100 pct)}(
///         horizontal{justify_content: FlexStart, flex_basis: unit!(10 pct)}(
///             tab_square[;focus], tab_square[;focus], tab_square[;focus]
///         ),
///         column_box(
///             column[;red](
///                 vertical(select_square, select_square),
///                 horizontal{flex_wrap: Wrap}[gray](
///                     square[;focus], square[;focus], square[;focus], square[;focus],
///                     square[;focus], square[;focus], square[;focus], square[;focus],
///                     square[;focus], square[;focus], square[;focus], square[;focus]
///                 ),
///                 horizontal{flex_wrap: Wrap}[gray](
///                     square[;focus], square[;focus], square[;focus], square[;focus],
///                     square[;focus], square[;focus], square[;focus], square[;focus]
///                 )
///             )
///         )
///     )
/// }
/// // Basically becomes
/// commands.spawn_bundle(NodeBundle {
///     style: Style { size: size!(100 pct, 100 pct), .. vertical.style },
///     .. vertical
/// })
///   .with_children(|cmds| {
///     cmds.spawn_bundle(NodeBundle {
///         style: Style {justify_content: FlexStart, flex_basis: unit!(10 pct), .. horizontal.style },
///         .. horizontal
///     })
///       .with_children(|cmds| {
///         cmds.spawn_bundle(tab_square).insert(focus);
///         cmds.spawn_bundle(tab_square).insert(focus);
///         cmds.spawn_bundle(tab_square).insert(focus);
///       });
///     cmds.spawn_bundle(column_box)
///       .with_children(|cmds| {
///         cmds.spawn_bundle(column).insert(red)
///           .with_children(|cmds| {
///             vertical.with_children(|cmds| {
///               cmds.spawn_bundle(select_square);
///               cmds.spawn_bundle(select_square);
///             });
///             cmds.spawn_bundle(NodeBundle {
///                 style: Style {flex_wrap: Wrap, ..horizontal.style},
///                 .. horizontal
///             }).insert(gray)
///               .with_children(|cmds| {
///                 for _ in 0..12 {
///                   cmds.spawn_bundle(square).insert(focus);
///                 }
///               });
///             cmds.spawn_bundle(NodeBundle {
///                 style: Style {flex_wrap: Wrap, ..horizontal.style},
///                 .. horizontal
///             }).insert(gray)
///               .with_children(|cmds| {
///                 for _ in 0..8 {
///                   cmds.spawn_bundle(square).insert(focus);
///                 }
///               });
///           });
///       });
///   });
/// ```
#[macro_export]
macro_rules! build_ui {
    (@preset entity) => (());
    (@preset $anything_else:ident) => ($anything_else);
    (@preset $node:ident {$($styles:tt)*}) => (
        bevy::ui::entity::NodeBundle {
            style: style!(@default ($node.style.clone()) $($styles)*),
            .. $node.clone()
        }
    );
    (#[cmd($cmds:expr)] id ( $id:expr )) => ({
        use bevy::ecs::system::Insert;
        let parent = $cmds.parent_entity();
        let insert = bevy::ecs::system::Insert {
            entity: $id,
            component: bevy::prelude::Parent(parent),
        };
        $cmds.add_command(insert);
    });
    (#[cmd($cmds:expr)] $preset:ident
        $( {$($styles:tt)*} )? // {..} style modifiers
        $( [$($bundles:expr),* ; $($components:expr),*] )? // [..] components
        $( ( $( // (..) children
            $preset_chd:ident
                $( {$($styles_chd:tt)*} )? // {..} children style modifiers
                $( [$($components_chd:tt)*] )? // [..] children components
                $( ( $($chd_chd:tt)* ) )? // Children children
        ),* ) )?
    ) => (
        $cmds.spawn_bundle(build_ui!(@preset $preset $({$($styles)*})?).clone())
            $($(.insert_bundle($bundles.clone()))*
            $(.insert($components.clone()))*)?
            $(.with_children(|cmds| {
                $( build_ui!(
                    #[cmd(cmds)]
                    $preset_chd
                        $( { $($styles_chd)* } )?
                        $( [ $($components_chd)* ] )?
                        $( ( $($chd_chd)* ) )?
                ); )*
            }))?
    );
}
