mod slint_generatedMainWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_16_1 = slint :: VersionCheck_1_16_1 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFluentPalette_85 {
         r#accent_background : sp :: Property < slint :: Brush > , r#accent_control_border : sp :: Property < slint :: Brush > , r#background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , r#selection_background : sp :: Property < slint :: Brush > , r#selection_foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_85 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (_self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4284534271f64) as u32) as _)) as _ }
                     else {
                         _self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4278214584f64) as u32) as _) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_control_border () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                        ]))) as _ }
                     else {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                        ])) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_85_color_scheme = {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#color_scheme () }
                         . apply_pin (_self) . get () ;
                         if ! ((((r#tmp_FluentPalette_85_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Unknown) . clone ()))) {
                             (((((r#tmp_FluentPalette_85_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())))) as _ }
                         else {
                             (((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#foreground () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#selection_background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (_self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4278221012f64) as u32) as _))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#selection_foreground () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_accentify (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Color ,) -> sp :: Color {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let r#local_accent_color = sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . accent_color () ;
                 if ! ((((((r#local_accent_color) . clone () . to_argb_u8 ()) . r#alpha) . clone ()) as f64) > (((0f64) . clone ()) as f64)) {
                     (args . 0 . clone ()) as _ }
                 else {
                     {
                         let r#local_default_lch = (args . 0 . clone ()) . clone () . to_oklch () ;
                         let r#local_accent_lch = (r#local_accent_color) . clone () . to_oklch () ;
                         {
                             let l : f32 = (((r#local_default_lch) . r#lightness) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             let c : f32 = (((r#local_accent_lch) . r#chroma) . clone () as f32) . max (0.) as f32 ;
                             let alpha : f32 = ((1f64) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             sp :: Color :: from_oklch (l , c , ((r#local_accent_lch) . r#hue) . clone () as f32 , alpha) }
                         }
                     }
                 }
            ) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEditBase_root_1 {
         r#root_1 : sp :: r#Empty , r#root_clip_2 : sp :: r#Clip , r#placeholder_3 : sp :: r#ComplexText , r#contextmenuinternal_4 : sp :: r#ContextMenu , r#text_input_5 : sp :: r#TextInput , r#root_1_has_focus : sp :: Property < bool > , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_input_type : sp :: Property < sp :: r#InputType > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_margin : sp :: Property < sp :: LogicalLength > , r#root_1_password_revealed : sp :: Property < bool > , r#root_1_placeholder_3_horizontal_stretch : sp :: Property < f32 > , r#root_1_placeholder_3_max_height : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_3_vertical_stretch : sp :: Property < f32 > , r#root_1_placeholder_color : sp :: Property < slint :: Brush > , r#root_1_placeholder_text : sp :: Property < sp :: SharedString > , r#root_1_text_color : sp :: Property < slint :: Brush > , r#root_1_text_input_5_computed_x : sp :: Property < sp :: LogicalLength > , r#root_1_text_input_5_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_input_5_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_input_5_x : sp :: Property < sp :: LogicalLength > , r#root_1_width : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_edited : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_key_pressed : sp :: Callback < (slint :: language :: KeyEvent ,) , sp :: r#EventResult > , r#root_1_key_released : sp :: Callback < (slint :: language :: KeyEvent ,) , sp :: r#EventResult > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_input_type ()) . apply_pin (_self) . set ({
                 (sp :: r#InputType :: r#Text) as sp :: r#InputType }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = (1f64) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_0) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_max_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_min_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#stretch = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_horizontal_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_1) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_1) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_1) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_1) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_1) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_max_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#stretch = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_password_revealed ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_horizontal_stretch ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_max_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_max_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_3_vertical_stretch ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) - (((1f64) . clone ()) as f64)) as sp :: Coord) . max ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_computed_x ()) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_color ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . set ({
                 ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . get ()) as sp :: SharedString }
            ) ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . set ({
                 ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . get ()) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: SharedString :: from ("")) . clone ())))) . clone ())) && ((((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: SharedString :: from ("")) . clone ())))) . clone ())) {
                         ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_text ()) . apply_pin (_self) . get ()) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_72 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let window_adapter = & _self . globals . get () . unwrap () . window_adapter_impl () ;
                             let menu_item_tree_instance = InnerComponent_empty_6 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () ;
                             let context_menu_item_tree = sp :: VRc :: new (sp :: MenuFromItemTree :: new (sp :: VRc :: into_dyn (menu_item_tree_instance))) ;
                             let context_menu_item_tree_ = context_menu_item_tree . clone () ;
                             {
                                 let mut entries = sp :: SharedVector :: default () ;
                                 sp :: Menu :: sub_menu (& * context_menu_item_tree , sp :: Option :: None , & mut entries) ;
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . set (sp :: ModelRc :: new (sp :: SharedVectorModel :: from (entries))) ;
                                 let context_menu_item_tree = context_menu_item_tree_ . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu ()) . apply_pin (_self) . set_handler (move | entry | {
                                     let mut entries = sp :: SharedVector :: default () ;
                                     sp :: Menu :: sub_menu (& * context_menu_item_tree , sp :: Option :: Some (& entry . 0) , & mut entries) ;
                                     sp :: ModelRc :: new (sp :: SharedVectorModel :: from (entries)) }
                                ) ;
                                 let context_menu_item_tree = context_menu_item_tree_ . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_activated ()) . apply_pin (_self) . set_handler (move | entry | {
                                     sp :: Menu :: activate (& * context_menu_item_tree_ , & entry . 0) ;
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_close ()) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 ()) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             let context_menu_item_tree = sp :: VRc :: into_dyn (context_menu_item_tree) ;
                             if ! sp :: WindowInner :: from_pub (window_adapter . window ()) . show_native_popup_menu (context_menu_item_tree , position , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) {
                                 if let Some (current_id) = (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 ()) . apply_pin (_self) . popup_id . take () {
                                     sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                     }
                                 let id = sp :: WindowInner :: from_pub (window_adapter . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , true ,) ;
                                 (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 ()) . apply_pin (_self) . popup_id . set (Some (id)) ;
                                 InnerPopupMenuImpl_root_72 :: user_init (popup_instance_vrc) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_accepted ()) . apply_pin (_self) . call (& (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_color ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((((((((args . 0 . clone ()) . r#x) . clone ()) as f64) + ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_computed_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) < ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) {
                                 ({
                                     (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_computed_x ()) . apply_pin (_self) . set (sp :: LogicalLength :: new ((((((- (args . 0 . clone ()) . r#x)) . clone ()) as f64) + ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if ((((((((args . 0 . clone ()) . r#x) . clone ()) as f64) + ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_computed_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) > ((((((((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) - (((1f64) . clone ()) as f64))) . clone ()) as f64) {
                                     ({
                                         (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_computed_x ()) . apply_pin (_self) . set (sp :: LogicalLength :: new ((((((((((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((args . 0 . clone ()) . r#x) . clone ()) as f64))) . clone ()) as f64) - ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) - (((1f64) . clone ()) as f64)) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#edited ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_edited ()) . apply_pin (_self) . call (& (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_password_revealed ()) . apply_pin (_self) . get () {
                         (sp :: r#InputType :: r#Text) as _ }
                     else {
                         (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_input_type ()) . apply_pin (_self) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_key_pressed ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#key_released ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_key_released ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#selection_background () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#selection_foreground () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((1f64) . clone ()) as f64)) as sp :: Coord) . max ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_width ()) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#page_height ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if ! (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) . get () {
                         ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_password_revealed ()) . apply_pin (_self) . set (false as _)) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (1f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => (((((((1f64) . clone ()) as f64) * ((((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . r#clear_selection (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . r#copy (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . r#cut (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . r#paste (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . r#select_all (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ((InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (_self) . set_selection_offsets (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_6 {
         r#empty_6 : sp :: r#Empty , r#menuitem_7 : sp :: r#MenuItem , r#menuitem_8 : sp :: r#MenuItem , r#menuitem_9 : sp :: r#MenuItem , r#menuitem_10 : sp :: r#MenuItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_empty_6 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEditBase_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_6 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#cut (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 4u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((! _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) . clone ())) && (((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate (((sp :: SharedString :: from ("Cut")) . clone ()) as _ , ((sp :: SharedString :: from ("LineEditBase")) . clone ()) as _ , ((sp :: SharedString :: from ("vpn-app")) . clone ()) as _ , ((sp :: Slice :: from_slice (& [])) . clone ()) as _ , ((1f64) . clone ()) as _ , ((sp :: SharedString :: from ("")) . clone ()) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#copy (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 4u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone () . is_empty ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate (((sp :: SharedString :: from ("Copy")) . clone ()) as _ , ((sp :: SharedString :: from ("LineEditBase")) . clone ()) as _ , ((sp :: SharedString :: from ("vpn-app")) . clone ()) as _ , ((sp :: Slice :: from_slice (& [])) . clone ()) as _ , ((1f64) . clone ()) as _ , ((sp :: SharedString :: from ("")) . clone ()) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#paste (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 4u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((! _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) . clone ())) && (((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate (((sp :: SharedString :: from ("Paste")) . clone ()) as _ , ((sp :: SharedString :: from ("LineEditBase")) . clone ()) as _ , ((sp :: SharedString :: from ("vpn-app")) . clone ()) as _ , ((sp :: Slice :: from_slice (& [])) . clone ()) as _ , ((1f64) . clone ()) as _ , ((sp :: SharedString :: from ("")) . clone ()) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#select_all (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 4u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone () . is_empty ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate (((sp :: SharedString :: from ("Select All")) . clone ()) as _ , ((sp :: SharedString :: from ("LineEditBase")) . clone ()) as _ , ((sp :: SharedString :: from ("vpn-app")) . clone ()) as _ , ((sp :: Slice :: from_slice (& [])) . clone ()) as _ , ((1f64) . clone ()) as _ , ((sp :: SharedString :: from ("")) . clone ()) as _)) as _ }
                ) ;
                 }
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#shortcut ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#shortcut ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#shortcut ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 () + sp :: r#MenuItem :: FIELD_OFFSETS . r#shortcut ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#empty_6 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_empty_6 :: FIELD_OFFSETS . r#empty_6 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_empty_6 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEditBase_root_1 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEditBase_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_empty_6 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_empty_6 :: FIELD_OFFSETS . r#empty_6 ()) , sp :: VOffset :: new (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_7 ()) , sp :: VOffset :: new (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_8 ()) , sp :: VOffset :: new (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_9 ()) , sp :: VOffset :: new (InnerComponent_empty_6 :: FIELD_OFFSETS . r#menuitem_10 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_empty_6) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_empty_6 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_empty_6 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_empty_6 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_6 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some (parent_rc) = self . parent . clone () . upgrade () {
                 let parent_origin = sp :: VRcMapped :: origin (& parent_rc) ;
                 * _result = sp :: ItemRc :: new_root (parent_origin) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEditClearIcon_root_11 {
         r#root_11 : sp :: r#ClippedImage , r#toucharea_12 : sp :: r#TouchArea , r#root_11_x : sp :: Property < sp :: LogicalLength > , r#root_11_clear : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditClearIcon_root_11 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditClearIcon_root_11 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as _ }
                ) ;
                 }
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11_clear ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEditPasswordIcon_root_13 {
         r#root_13 : sp :: r#ClippedImage , r#toucharea_14 : sp :: r#TouchArea , r#root_13_hide_password_image : sp :: Property < sp :: Image > , r#root_13_show_password : sp :: Property < bool > , r#root_13_show_password_image : sp :: Property < sp :: Image > , r#root_13_x : sp :: Property < sp :: LogicalLength > , r#root_13_clicked : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditPasswordIcon_root_13 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditPasswordIcon_root_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password ()) . apply_pin (_self) . get () {
                         ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_hide_password_image ()) . apply_pin (_self) . get ()) as _ }
                     else {
                         (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password_image ()) . apply_pin (_self) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as _ }
                ) ;
                 }
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#toucharea_14 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password ()) . apply_pin (_self) . set ((! (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password ()) . apply_pin (_self) . get ()) as _) ;
                             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_clicked ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#toucharea_14 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#toucharea_14 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#toucharea_14 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEdit_root_15 {
         r#root_15 : sp :: r#Empty , r#background_16 : sp :: r#BasicBorderRectangle , r#focus_border_23 : sp :: r#BasicBorderRectangle , r#base_18 : InnerLineEditBase_root_1 , r#root_15_background_16_width : sp :: Property < sp :: LogicalLength > , r#root_15_height : sp :: Property < sp :: LogicalLength > , r#root_15_layout_17_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_15_layout_17_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_layout_17_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_state : sp :: Property < i32 > , r#root_15_width : sp :: Property < sp :: LogicalLength > , r#root_15_x : sp :: Property < sp :: LogicalLength > , r#root_15_y : sp :: Property < sp :: LogicalLength > , r#root_15_accessible_action_set_value : sp :: Callback < (sp :: SharedString ,) , () > , repeater0 : sp :: Conditional < InnerComponent_lineeditclearicon_19 > , repeater1 : sp :: Conditional < InnerComponent_lineeditpasswordicon_21 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_15 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_15 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((((((((((((! ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone () . is_empty ())) . clone ())) && ((((((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_input_type ()) . apply_pin (_self) . get ()) . clone ())) != (((sp :: r#InputType :: r#Password) . clone ())))) . clone ())))) . clone ())) && ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())) && ((((! (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (_self) . get ())) . clone ())))) . clone ())) && ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((((((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_input_type ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#InputType :: r#Password) . clone ())))) . clone ())) && ((((! ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone () . is_empty ())) . clone ())))) . clone ())) && ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             InnerLineEditBase_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 6u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_accessible_action_set_value ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set (args . 0 . clone () as _) ;
                             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_edited ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_background_16_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         4usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (1f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [2usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [2usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (12f64) . clone () as _ ;
                                 the_struct . r#end = (12f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_background_16_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (1f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (12f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_input_5_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_2 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_2) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_2) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((160f64 as sp :: Coord) . max (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_2) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_2) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_3 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_3) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_3) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_3) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_3) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_has_focus ()) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_15_state = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_15_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (slint :: Brush :: SolidColor (if {
                                 * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_15_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((3005095454f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9998f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((2332033023f64) as u32) , position : 1f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((2332033023f64) as u32) , position : 1f64 as _ }
                        ]))) as _ }
                     else {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9999f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((1929379840f64) as u32) , position : 1f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((1929379840f64) as u32) , position : 1f64 as _ }
                        ])) }
                    ) as _ }
                ) ;
                 }
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#selection_background () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                        ) . color ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                        ) . color () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((2f64) . clone () as f64)) {
                         ({
                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_margin ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_h ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((160f64 as sp :: Coord) . max (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditclearicon_19 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerLineEdit_root_15 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_lineeditpasswordicon_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => (((2f64) . clone ()) . clone () as sp :: Coord , (((((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((4f64) . clone ()) . clone () as sp :: Coord , (((((((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 6u32 ..= 9u32 => return InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . item_geometry (index - 6u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#TextInput , 2u32 => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessible_role (0) , 6u32 ..= 9u32 => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessible_role (index - 6u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_text ()) . apply_pin (_self) . get ()) , (0u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some ((InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (2u32 , _) => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessible_string_property (0 , what) , (6u32 ..= 9u32 , _) => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessible_string_property (index - 6u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_accessible_action_set_value ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                 (2u32 , _) => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessibility_action (0 , action) , (6u32 ..= 9u32 , _) => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . accessibility_action (index - 6u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#SetValue , 2u32 => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . supported_accessibility_actions (0) , 6u32 ..= 9u32 => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . supported_accessibility_actions (index - 6u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 6u32 ..= 9u32 => InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (_self) . item_element_infos (index - 6u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_lineeditclearicon_19 {
         r#lineeditclearicon_19 : InnerLineEditClearIcon_root_11 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_lineeditclearicon_19 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_lineeditclearicon_19 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditClearIcon_root_11 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11_clear ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . set (sp :: SharedString :: from ("") as _)) ;
                                 }
                             ;
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_edited ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . call (& ((sp :: SharedString :: from ("")) . clone () as _ ,))) ;
                                 }
                             ;
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () . apply_pin (x . as_pin_ref ()) . r#fn_focus ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEditClearIcon_root_11 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#lineeditclearicon_19 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#lineeditclearicon_19 . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (16f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (16f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#lineeditclearicon_19 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#lineeditclearicon_19 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((16f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , 0u32 => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_lineeditclearicon_19 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_lineeditclearicon_19 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#root_11 ()) , sp :: VOffset :: new (InnerComponent_lineeditclearicon_19 :: FIELD_OFFSETS . r#lineeditclearicon_19 () + InnerLineEditClearIcon_root_11 :: FIELD_OFFSETS . r#toucharea_12 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_lineeditclearicon_19) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_lineeditclearicon_19 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_lineeditclearicon_19 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_lineeditclearicon_19 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_lineeditclearicon_19 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_lineeditclearicon_19 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_lineeditclearicon_19 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_lineeditpasswordicon_21 {
         r#lineeditpasswordicon_21 : InnerLineEditPasswordIcon_root_13 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_lineeditpasswordicon_21 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_lineeditpasswordicon_21 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditPasswordIcon_root_13 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_password_revealed ()) . apply_pin (x . as_pin_ref ())) . map (| x | sp :: Property :: link_two_way ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password ()) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_hide_password_image ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password_image ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) * (((1f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [4usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_hide_password_image ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13_show_password_image ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEditPasswordIcon_root_13 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#lineeditpasswordicon_21 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#lineeditpasswordicon_21 . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((((((((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) * (((1f64) . clone ()) as f64))) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((((((((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) * (((1f64) . clone ()) as f64))) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#lineeditpasswordicon_21 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#lineeditpasswordicon_21 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , (((((((((InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . get ()) . clone () . size ()) . r#width) . clone ()) as f64) * (((1f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [4usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , 0u32 => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_lineeditpasswordicon_21 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerLineEdit_root_15 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_lineeditpasswordicon_21 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#root_13 ()) , sp :: VOffset :: new (InnerComponent_lineeditpasswordicon_21 :: FIELD_OFFSETS . r#lineeditpasswordicon_21 () + InnerLineEditPasswordIcon_root_13 :: FIELD_OFFSETS . r#toucharea_14 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_lineeditpasswordicon_21) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_lineeditpasswordicon_21 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_lineeditpasswordicon_21 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_lineeditpasswordicon_21 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_lineeditpasswordicon_21 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_lineeditpasswordicon_21 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_lineeditpasswordicon_21 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_24 {
         r#root_24 : sp :: r#BasicBorderRectangle , r#rectangle_25 : sp :: r#BasicBorderRectangle , r#root_24_height : sp :: Property < sp :: LogicalLength > , r#root_24_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_24 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_24 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((((((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((((((InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_26 {
         r#root_26 : sp :: r#Empty , r#i_background_27 : sp :: r#BasicBorderRectangle , r#i_border_28 : sp :: r#BasicBorderRectangle , r#i_touch_area_34 : sp :: r#TouchArea , r#i_focus_scope_35 : sp :: r#FocusScope , r#root_26_checked : sp :: Property < bool > , r#root_26_has_focus : sp :: Property < bool > , r#root_26_height : sp :: Property < sp :: LogicalLength > , r#root_26_i_background_27_width : sp :: Property < sp :: LogicalLength > , r#root_26_i_layout_29_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_26_i_layout_29_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_i_layout_29_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_icon : sp :: Property < sp :: Image > , r#root_26_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_pressed : sp :: Property < bool > , r#root_26_primary : sp :: Property < bool > , r#root_26_state : sp :: Property < i32 > , r#root_26_text : sp :: Property < sp :: SharedString > , r#root_26_text_color : sp :: Property < slint :: Brush > , r#root_26_width : sp :: Property < sp :: LogicalLength > , r#root_26_x : sp :: Property < sp :: LogicalLength > , r#root_26_y : sp :: Property < sp :: LogicalLength > , r#root_26_accessible_action_default : sp :: Callback < () , () > , r#root_26_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_30 > , repeater1 : sp :: Conditional < InnerComponent_text_32 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_36 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_26 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_26 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((((((((sp :: Image :: default ()) . clone () . size ()) . r#width) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())) && (((((((((sp :: Image :: default ()) . clone () . size ()) . r#height) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text ()) . apply_pin (_self) . get ()) . clone ())) != (((sp :: SharedString :: from ("")) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_has_focus ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_accessible_action_default ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_has_focus ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_background_27_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         4usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [2usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [2usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (12f64) . clone () as _ ;
                                 the_struct . r#end = (12f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_background_27_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (4f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (12f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (5f64) . clone () as _ ;
                             the_struct . r#end = (5f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_4 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_4) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_4) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_4) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_4) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_5 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_5) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_5) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_5) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_5) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_pressed ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_pressed ()) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if (InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_state = (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_state = (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                     (({
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) . clone () . with_alpha ((0.8f64) . clone () as f32)) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                     (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                         (({
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) . clone () . with_alpha ((0.9f64) . clone () as f32)) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                         ({
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                                     else {
                                         if (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get () {
                                             ({
                                                 * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_background () }
                                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                                         else {
                                             slint :: Brush :: SolidColor (if {
                                                 * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                                 (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                             else {
                                                 sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                            ) }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_state = (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) . clone ())) {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_26_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     ({
                                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_control_border () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                                 else {
                                     if (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) . get () {
                                         ({
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#accent_control_border () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                                     else {
                                         if {
                                             * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                                             (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                             , sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                            ]))) as _ }
                                         else {
                                             slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                             , sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                            ])) }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . set ((! (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_clicked ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from (" ")) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\n")) . clone ())))) . clone ()))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     (InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_36 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_h ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_36 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_26 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_36 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text ()) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_accessible_action_default ()) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_30 {
         r#image_30 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_30 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_30 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_icon ()) . apply_pin (x . as_pin_ref ())) . map (| x | sp :: Property :: link_two_way ((InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (20f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_30 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_30 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_image_30 :: FIELD_OFFSETS . r#image_30 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_30) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_30 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_30 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_30 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_30 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_30 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_30 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_32 {
         r#text_32 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_32 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_32 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_32 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_32 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_32 :: FIELD_OFFSETS . r#text_32 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_32) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_32 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_32 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_32 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_32 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_32 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_32 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_36 {
         r#focusborder_36 : InnerFocusBorder_root_24 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_36 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_36 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_24 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_24 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_36 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_36 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_36 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_36 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_36 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_26 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_36 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#root_24 ()) , sp :: VOffset :: new (InnerComponent_focusborder_36 :: FIELD_OFFSETS . r#focusborder_36 () + InnerFocusBorder_root_24 :: FIELD_OFFSETS . r#rectangle_25 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_36) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_36 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_36 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_36 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_36 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_36 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_36 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItemBase_root_38 {
         r#root_38 : sp :: r#Empty , r#background_layer_39 : sp :: r#BasicBorderRectangle , r#touch_area_visibility_40 : sp :: r#Clip , r#touch_area_41 : sp :: r#TouchArea , r#rectangle_43 : sp :: r#Empty , r#image_46 : sp :: r#ImageItem , r#label_Opacity_47 : sp :: r#Opacity , r#label_48 : sp :: r#SimpleText , r#shortcut_49 : sp :: r#SimpleText , r#root_38_alternate_foreground : sp :: Property < slint :: Brush > , r#root_38_background_layer_39_height : sp :: Property < sp :: LogicalLength > , r#root_38_background_layer_39_width : sp :: Property < sp :: LogicalLength > , r#root_38_current_background : sp :: Property < slint :: Brush > , r#root_38_current_foreground : sp :: Property < slint :: Brush > , r#root_38_default_foreground : sp :: Property < slint :: Brush > , r#root_38_entry : sp :: Property < sp :: MenuEntry > , r#root_38_height : sp :: Property < sp :: LogicalLength > , r#root_38_horizontal_padding : sp :: Property < sp :: LogicalLength > , r#root_38_icon_size : sp :: Property < sp :: LogicalLength > , r#root_38_image_46_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_38_image_46_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_38_image_46_y : sp :: Property < sp :: LogicalLength > , r#root_38_is_current : sp :: Property < bool > , r#root_38_label_48_font_metrics : sp :: Property < sp :: FontMetrics > , r#root_38_layout_42_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_38_layout_42_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_38_layout_42_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_38_layout_42_spacing : sp :: Property < sp :: LogicalLength > , r#root_38_separator_color : sp :: Property < slint :: Brush > , r#root_38_state : sp :: Property < i32 > , r#root_38_sub_menu_icon : sp :: Property < sp :: Image > , r#root_38_touch_area_41_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_38_touch_area_41_height : sp :: Property < sp :: LogicalLength > , r#root_38_touch_area_41_width : sp :: Property < sp :: LogicalLength > , r#root_38_width : sp :: Property < sp :: LogicalLength > , r#root_38_x : sp :: Property < sp :: LogicalLength > , r#root_38_y : sp :: Property < sp :: LogicalLength > , r#root_38_activate : sp :: Callback < (sp :: MenuEntry , sp :: Coord ,) , () > , r#root_38_clear_current : sp :: Callback < () , () > , r#root_38_set_current : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_text_44 > , repeater1 : sp :: Conditional < InnerComponent_image_50 > , repeater2 : sp :: Conditional < InnerComponent_rectangle_52 > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItemBase_root_38 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#checked) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#has_sub_menu) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_alternate_foreground ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_default_foreground ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_background_layer_39_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_background_layer_39_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - ((({
                         let r#tmp_root_38_label_48_font_metrics = (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_label_48_font_metrics ()) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_38_label_48_font_metrics) . r#ascent) . clone ()) as f64) - ((((r#tmp_root_38_label_48_font_metrics) . r#descent) . clone ()) as f64) }
                    ) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_label_48_font_metrics ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 ()) . apply_pin (_self) . font_metrics (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (3usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 7u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                 the_struct . r#end = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_spacing ()) . apply_pin (_self) . get () . get ()) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (3usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 7u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#end = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (3usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 7u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_spacing ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ! ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#enabled {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_absolute_position ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((((r#parent_position) . r#x) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct . r#y = ((((((r#parent_position) . r#y) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#background_layer_39 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_current_background ()) . apply_pin (_self) . get ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (! ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#enabled) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((((((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Move) . clone ())))) . clone ())) && ((((! (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) . get ())) . clone ())) {
                                 ({
                                     (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_set_current ()) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 if ((((((((((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Down) . clone ())))) . clone ())) && (((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#has_sub_menu) . clone ())))) . clone ())) && (((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#enabled) . clone ())) {
                                     ({
                                         (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_activate ()) . apply_pin (_self) . call (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . clone () as _ , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_absolute_position ()) . apply_pin (_self) . get ()) . r#y) . clone () as _ ,)) }
                                    ) ;
                                     }
                                 else {
                                     if ((((((((((((((((((((((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Up) . clone ())))) . clone ())) && ((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())))) . clone ())) && ((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) < ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ())))) . clone ())) && ((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())))) . clone ())) && ((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) < ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ())))) . clone ())) && (((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#enabled) . clone ())) {
                                         ({
                                             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_activate ()) . apply_pin (_self) . call (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . clone () as _ , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_touch_area_41_absolute_position ()) . apply_pin (_self) . get ()) . r#y) . clone () as _ ,)) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_38_label_48_font_metrics = (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_label_48_font_metrics ()) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_38_label_48_font_metrics) . r#ascent) . clone ()) as f64) - ((((r#tmp_root_38_label_48_font_metrics) . r#descent) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#icon) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_Opacity_47 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((2f64) . clone () as f64)) {
                         (0.5f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_current_foreground ()) . apply_pin (_self) . get ()) as _ }
                     else {
                         (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_default_foreground ()) . apply_pin (_self) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_background_layer_39_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#title) as _ }
                ) ;
                 }
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_alternate_foreground ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_background_layer_39_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Right) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ToSharedString :: to_shared_string (& (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#shortcut) . clone ())) as _ }
                ) ;
                 }
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#background_layer_39 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#background_layer_39 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if ((((! (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get ())) . clone ())) && ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) . get ()) . clone ())) {
                         ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_clear_current ()) . apply_pin (_self) . call (& ())) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_44 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_52 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())) , sp :: Orientation :: Vertical => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_44 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_52 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_44 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_50 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerMenuItemBase_root_38 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_52 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 7u32 => ((((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 10u32 => ((({
                     let r#tmp_root_38_label_48_font_metrics = (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_label_48_font_metrics ()) . apply_pin (_self) . get () ;
                     ((((r#tmp_root_38_label_48_font_metrics) . r#ascent) . clone ()) as f64) - ((((r#tmp_root_38_label_48_font_metrics) . r#descent) . clone ()) as f64) }
                ) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_image_46_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 11u32 => ((((((((((((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 7u32 => sp :: r#AccessibleRole :: r#Text , 11u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (7u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (11u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (((InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#title) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_44 {
         r#text_44 : sp :: r#SimpleText , r#text_44_min_height : sp :: Property < sp :: LogicalLength > , r#text_44_min_width : sp :: Property < sp :: LogicalLength > , r#text_44_preferred_height : sp :: Property < sp :: LogicalLength > , r#text_44_preferred_width : sp :: Property < sp :: LogicalLength > , r#text_44_x : sp :: Property < sp :: LogicalLength > , r#text_44_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_44 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_44 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#foreground () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_preferred_height ()) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_min_height ()) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("✓")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_preferred_width ()) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_min_width ()) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - ((((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - ((((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("✓")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_44 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_44 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_44 :: FIELD_OFFSETS . r#text_44 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_44) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_44 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_44 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_44 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_44 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 9u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_44 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_44 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_50 {
         r#image_50 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_50 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_50 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_background_layer_39_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_sub_menu_icon ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) as f64) - (((0f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [6usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_50 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_50 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_image_50 :: FIELD_OFFSETS . r#image_50 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_50) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_50 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_50 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_50 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_50 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 8u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_50 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_50 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_52 {
         r#rectangle_52 : sp :: r#Rectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_52 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_52 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_rectangle_52 :: FIELD_OFFSETS . r#rectangle_52 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_separator_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_rectangle_52 :: FIELD_OFFSETS . r#rectangle_52 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_rectangle_52 :: FIELD_OFFSETS . r#rectangle_52 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((1f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((1f64) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_rectangle_52 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_38 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_52 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_rectangle_52 :: FIELD_OFFSETS . r#rectangle_52 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_52) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_52 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_52 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_52 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_52 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_52 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_52 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItem_root_54 {
         r#root_54 : sp :: r#Empty , r#base_56 : InnerMenuItemBase_root_38 , r#root_54_empty_55_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_54_empty_55_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_54_empty_55_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_54_height : sp :: Property < sp :: LogicalLength > , r#root_54_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_54_max_height : sp :: Property < sp :: LogicalLength > , r#root_54_min_height : sp :: Property < sp :: LogicalLength > , r#root_54_width : sp :: Property < sp :: LogicalLength > , r#root_54_x : sp :: Property < sp :: LogicalLength > , r#root_54_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItem_root_54 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItem_root_54 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItemBase_root_38 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 2u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                                 (0f64) as _ }
                             else {
                                 1f64 }
                            ) . clone () as _ ;
                             the_struct . r#end = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                                 (0f64) as _ }
                             else {
                                 1f64 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                             (0f64) as _ }
                         else {
                             1f64 }
                        ) . clone () as _ ;
                         the_struct . r#end = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                             (0f64) as _ }
                         else {
                             1f64 }
                        ) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                             (4f64) as _ }
                         else {
                             1f64 }
                        ) . clone () as _ ;
                         the_struct . r#end = (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                             (4f64) as _ }
                         else {
                             1f64 }
                        ) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_base_56_entry = (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get () ;
                         ((({
                             let r#layout_info_6 = {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (if (r#tmp_base_56_entry) . r#is_separator {
                                     (9f64) as _ }
                                 else {
                                     (((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ()))) . r#max }
                                ) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info_6) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (if (r#tmp_base_56_entry) . r#is_separator {
                                     (9f64) as _ }
                                 else {
                                     (32f64 as sp :: Coord) . max ((((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((((({
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                                         the_struct . r#min = (0f64) . clone () as _ ;
                                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                                         the_struct . r#preferred = (0f64) . clone () as _ ;
                                         the_struct . r#stretch = (1f64) . clone () as _ ;
                                         the_struct }
                                    ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ()))) . r#min as sp :: Coord) }
                                ) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info_6) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info_6) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info_6) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_max_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                         (9f64) as _ }
                     else {
                         (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ()))) . r#max }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                         (9f64) as _ }
                     else {
                         (32f64 as sp :: Coord) . max ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ()))) . r#min as sp :: Coord) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_alternate_foreground ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#background_layer_39 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_current_background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((167772160f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_current_foreground ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_default_foreground ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_54_empty_55_padding = if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                             (4f64) as _ }
                         else {
                             1f64 }
                         ;
                         ((((((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((r#tmp_root_54_empty_55_padding) . clone ()) as f64))) . clone ()) as f64) - (((r#tmp_root_54_empty_55_padding) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (11f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_separator_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_layout_42_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_sub_menu_icon ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                         (4f64) as _ }
                     else {
                         1f64 }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_horizontal_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_icon_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_sub_menu_icon ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItemBase_root_38 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_max_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((if ((InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) . get ()) . r#is_separator {
                     (4f64) as _ }
                 else {
                     1f64 }
                ) . clone ()) . clone () as sp :: Coord ,) , 2u32 ..= 12u32 => return InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . item_geometry (index - 2u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessible_role (0) , 2u32 ..= 12u32 => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessible_role (index - 2u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , _) => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessible_string_property (0 , what) , (2u32 ..= 12u32 , _) => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessible_string_property (index - 2u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (1u32 , _) => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessibility_action (0 , action) , (2u32 ..= 12u32 , _) => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . accessibility_action (index - 2u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . supported_accessibility_actions (0) , 2u32 ..= 12u32 => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . supported_accessibility_actions (index - 2u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 ..= 12u32 => InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () . apply_pin (_self) . item_element_infos (index - 2u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerPopupMenuImpl_root_72 {
         r#root_72 : sp :: r#WindowItem , r#focus_scope_74 : sp :: r#FocusScope , r#frame_shadow_75 : sp :: r#BoxShadow , r#frame_76 : sp :: r#BasicBorderRectangle , r#frame_clip_77 : sp :: r#Clip , r#sub_menu_83 : sp :: r#ContextMenu , r#root_72_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_72_current_highlight : sp :: Property < i32 > , r#root_72_current_highlight_y_pos : sp :: Property < sp :: LogicalLength > , r#root_72_current_open : sp :: Property < i32 > , r#root_72_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_72_frame_76_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_72_layout_78_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_72_layout_78_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_72_layout_78_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_72_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_72_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_72_optimized_open_sub_menu_after_timeout_73_interval : sp :: Property < i64 > , r#root_72_optimized_open_sub_menu_after_timeout_73_running : sp :: Property < bool > , r#root_72_sub_menu_83_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_72_sub_menu_83_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_72_activated : sp :: Callback < (sp :: MenuEntry ,) , () > , r#root_72_close : sp :: Callback < () , () > , r#root_72_optimized_open_sub_menu_after_timeout_73_triggered : sp :: Callback < () , () > , r#root_72_sub_menu : sp :: Callback < (sp :: MenuEntry ,) , sp :: ModelRc < sp :: MenuEntry > > , repeater0 : sp :: Repeater < InnerComponent_menuitem_79 > , repeater1 : sp :: Repeater < InnerComponent_keybinding_81 > , change_tracker0 : sp :: ChangeTracker , change_tracker1 : sp :: ChangeTracker , timer0 : sp :: Timer , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerPopupMenuImpl_root_72 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_absolute_position ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((((r#parent_position) . r#x) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct . r#y = ((((((r#parent_position) . r#y) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set ({
                 (((- 1f64) as i32)) as i32 }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . set ({
                 (((- 1f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! []))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_frame_76_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_9 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_9) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_9) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((280f64 as sp :: Coord) . max (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_9) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_9) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_9) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (5f64) . clone () as _ ;
                                 the_struct . r#end = (5f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (5f64) . clone () as _ ;
                             the_struct . r#end = (5f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (5f64) . clone () as _ ;
                             the_struct . r#end = (5f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_frame_76_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_interval ()) . apply_pin (_self) . set ({
                 (500f64) as i64 }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_triggered ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . set (false as _) ;
                             if ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) >= (((0f64) . clone ()) as f64) {
                                 ({
                                     if (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                         x => {
                                             let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                             x . row_data_tracked (index) . unwrap_or_default () }
                                         }
                                    ) . r#has_sub_menu {
                                         ({
                                             _self . r#fn_activate (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                 x => {
                                                     let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                 }
                                             as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight_y_pos ()) . apply_pin (_self) . get () . get () as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get () as _) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . set ((((- 1f64)) as i32) as _) ;
                                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . r#close (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) }
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu_83_absolute_position ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((((r#parent_position) . r#x) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct . r#y = ((((((r#parent_position) . r#y) . clone ()) as f64) + (((0f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . set (false as _) ;
                                 let r#return_check_merge0 = if ((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f700}")) . clone ())) {
                                     ((((false) . clone ()) . clone () , (({
                                         if ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) < (((1f64) . clone ()) as f64) {
                                             ({
                                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set (((((((match & ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) . clone () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                ) . clone ()) as f64) - (((1f64) . clone ()) as f64))) as i32) as _) }
                                            ) ;
                                             }
                                         else {
                                             if (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                 x => {
                                                     let index = ((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) - (((1f64) . clone ()) as f64))) as i32)) as usize ;
                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                 }
                                            ) . r#is_separator {
                                                 ({
                                                     (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set ((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) - (((((2f64) as i32)) . clone ()) as f64)) as _) }
                                                ) ;
                                                 }
                                             else {
                                                 {
                                                     (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set ((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) - (((((1f64) as i32)) . clone ()) as f64)) as _) }
                                                 }
                                             }
                                         ;
                                         sp :: r#EventResult :: r#Accept }
                                    ) . clone ()) . clone () ,)) as _ }
                                 else {
                                     if ((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f701}")) . clone ())) {
                                         ((((false) . clone ()) . clone () , (({
                                             if ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) >= (((((((match & ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) . clone () {
                                                 x => {
                                                     x . model_tracker () . track_row_count_changes () ;
                                                     x . row_count () as i32 }
                                                 }
                                            ) . clone ()) as f64) - (((1f64) . clone ()) as f64))) . clone ()) as f64) {
                                                 ({
                                                     (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set (((0f64) as i32) as _) }
                                                ) ;
                                                 }
                                             else {
                                                 if (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                     x => {
                                                         let index = ((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) + (((1f64) . clone ()) as f64))) as i32)) as usize ;
                                                         x . row_data_tracked (index) . unwrap_or_default () }
                                                     }
                                                ) . r#is_separator {
                                                     ({
                                                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set ((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) + (((((2f64) as i32)) . clone ()) as f64)) as _) }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . set ((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) + (((((1f64) as i32)) . clone ()) as f64)) as _) }
                                                     }
                                                 }
                                             ;
                                             sp :: r#EventResult :: r#Accept }
                                        ) . clone ()) . clone () ,)) as _ }
                                     else {
                                         if ((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\n")) . clone ())) {
                                             ((((false) . clone ()) . clone () , (({
                                                 if ((((((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) >= (((0f64) . clone ()) as f64))) . clone ())) && ((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) < (((match & ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) . clone () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                ) . clone ()) as f64))) . clone ())))) . clone ())) && ((((match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                     x => {
                                                         let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                         x . row_data_tracked (index) . unwrap_or_default () }
                                                     }
                                                ) . r#enabled) . clone ())) {
                                                     ({
                                                         _self . r#fn_activate (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                             x => {
                                                                 let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                                 x . row_data_tracked (index) . unwrap_or_default () }
                                                             }
                                                         as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight_y_pos ()) . apply_pin (_self) . get () . get () as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get () as _) }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         }
                                                     }
                                                 ;
                                                 sp :: r#EventResult :: r#Accept }
                                            ) . clone ()) . clone () ,)) as _ }
                                         else {
                                             if ! (((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f703}")) . clone ()))) {
                                                 ({
                                                     if ((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f702}")) . clone ())) {
                                                         ({
                                                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_close ()) . apply_pin (_self) . call (& ()) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     (((true) . clone ()) . clone () , ((sp :: r#EventResult :: r#Reject) . clone ()) . clone () ,) }
                                                ) as _ }
                                             else {
                                                 (((false) . clone ()) . clone () , (({
                                                     if ((((((((((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) >= (((0f64) . clone ()) as f64))) . clone ())) && ((((((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) . clone ()) as f64) < (((match & ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get ()) . clone () {
                                                         x => {
                                                             x . model_tracker () . track_row_count_changes () ;
                                                             x . row_count () as i32 }
                                                         }
                                                    ) . clone ()) as f64))) . clone ())))) . clone ())) && ((((match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                         x => {
                                                             let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                             x . row_data_tracked (index) . unwrap_or_default () }
                                                         }
                                                    ) . r#has_sub_menu) . clone ())))) . clone ())) && ((((match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                         x => {
                                                             let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                             x . row_data_tracked (index) . unwrap_or_default () }
                                                         }
                                                    ) . r#enabled) . clone ())) {
                                                         ({
                                                             _self . r#fn_activate (match & (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . get () {
                                                                 x => {
                                                                     let index = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get ()) as usize ;
                                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                                 }
                                                             as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight_y_pos ()) . apply_pin (_self) . get () . get () as _ , (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (_self) . get () as _) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     sp :: r#EventResult :: r#Accept }
                                                ) . clone ()) . clone () ,) }
                                             }
                                         }
                                     }
                                 ;
                                 if (r#return_check_merge0) . 0 {
                                     (((({
                                         sp :: r#EventResult :: r#Reject }
                                    ) . clone ()) . clone () , ((true) . clone ()) . clone () , ((sp :: r#EventResult :: r#Reject) . clone ()) . clone () ,)) as _ }
                                 else {
                                     (((sp :: r#EventResult :: r#Reject) . clone ()) . clone () , ((false) . clone ()) . clone () , (((r#return_check_merge0) . 1) . clone ()) . clone () ,) }
                                 }
                             ;
                             if (r#returned_expression0) . 1 {
                                 ((r#returned_expression0) . 0) as _ }
                             else {
                                 (r#returned_expression0) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((1107296256f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((603979776f64) as u32) }
                    ) . color ()) as _ }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . set ((((- 1f64)) as i32) as _) ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_activated ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_close ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_72 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let window_adapter = & _self . globals . get () . unwrap () . window_adapter_impl () ;
                             let entries = (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu_83_entries ()) . apply_pin (_self) . get () ;
                             {
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_entries ()) . apply_pin (_self) . set (entries . clone ()) ;
                                 let self_weak = parent_weak . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu ()) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . sub_menu . call (entry) }
                                     else {
                                         :: core :: default :: Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_activated ()) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . activated . call (entry) }
                                     else {
                                         :: core :: default :: Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_close ()) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             if let Some (current_id) = (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . popup_id . take () {
                                 sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                 }
                             let id = sp :: WindowInner :: from_pub (window_adapter . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true ,) ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . popup_id . set (Some (id)) ;
                             InnerPopupMenuImpl_root_72 :: user_init (popup_instance_vrc) ;
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#sub_menu ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_interval ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . r#fn_focus () ;
             {
                 }
             ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_interval ()) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker1 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             _self . update_timers () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_keybinding_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layoutinfo_h ()) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layoutinfo_v ()) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_keybinding_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_keybinding_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         fn update_timers (self : :: core :: pin :: Pin < & Self >) {
             let _self = self ;
             if (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . get () {
                 let interval = :: core :: time :: Duration :: from_millis ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_interval ()) . apply_pin (_self) . get () as u64) ;
                 if ! self . timer0 . running () || interval != self . timer0 . interval () {
                     let self_weak = self . self_weak . get () . unwrap () . clone () ;
                     self . timer0 . start (sp :: TimerMode :: Repeated , interval , move || {
                         if let Some (self_rc) = self_weak . upgrade () {
                             let _self = self_rc . as_pin_ref () ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_triggered ()) . apply_pin (_self) . call (& ()) }
                         }
                    ) ;
                     }
                 }
             else {
                 self . timer0 . stop () ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_activate (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: MenuEntry , arg_1 : sp :: Coord , arg_2 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 , arg_2 ,) ;
             ({
                 (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (_self) . set (false as _) ;
                 if (args . 0 . clone ()) . r#has_sub_menu {
                     (if (((! sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((args . 2 . clone ()) . clone () as f64))) . clone ())) || ((((! (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) . apply_pin (_self) . r#is_open (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)))) . clone ())) {
                         ({
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . set (args . 2 . clone () as _) ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu_83_entries ()) . apply_pin (_self) . set ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) as _) ;
                             (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 () + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show ()) . apply_pin (_self) . call (& (({
                                 let mut the_struct = slint :: LogicalPosition :: default () ;
                                 the_struct . r#x = ((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                 the_struct . r#y = (((((args . 1 . clone ()) . clone ()) as f64) - (((((InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_sub_menu_83_absolute_position ()) . apply_pin (_self) . get ()) . r#y) . clone ()) as f64))) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ ,)) }
                        ) ;
                         }
                     else {
                         {
                             }
                         }
                    ) ;
                     }
                 else {
                     {
                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_open ()) . apply_pin (_self) . set (((- 1f64) as i32) as _) ;
                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_activated ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) ;
                         (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_close ()) . apply_pin (_self) . call (& ()) }
                     }
                 }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_menuitem_79 {
         r#menuitem_79 : InnerMenuItem_root_54 , r#model_data : sp :: Property < sp :: MenuEntry > , r#model_index : sp :: Property < i32 > , r#menuitem_79_absolute_position : sp :: Property < slint :: LogicalPosition > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_menuitem_79 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_menuitem_79 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItem_root_54 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79_absolute_position ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#menuitem_79 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#menuitem_79 . tree_index . get ())) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((((r#parent_position) . r#x) . clone ()) as f64) + (((5f64) . clone ()) as f64))) . clone () as _ ;
                             the_struct . r#y = ((((((r#parent_position) . r#y) . clone ()) as f64) + ((((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone () as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_activate ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | x . as_pin_ref () . r#fn_activate (args . 0 . clone () as _ , args . 1 . clone () as _ , (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_clear_current ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . set ((((- 1f64)) as i32) as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_entry ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ApproxEq :: < f64 > :: approx_eq (& ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone () as f64) , & (((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as f64))) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_set_current ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 1u32 - 1) , true , sp :: FocusReason :: Programmatic) ;
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . set ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as _)) ;
                                 }
                             ;
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_optimized_open_sub_menu_after_timeout_73_running ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . set (true as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_layout_78_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_x ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItem_root_54 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (x)) ,) ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38_is_current ()) . apply_pin (_self) . get () {
                         ({
                             let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_current_highlight_y_pos ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . set (sp :: LogicalLength :: new (((((((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79_absolute_position ()) . apply_pin (_self) . get ()) . r#y) . clone ()) as f64) - ((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_absolute_position ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . r#y) . clone ()) as f64)) as sp :: Coord) as _)) ;
                             }
                        ) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_empty_55_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_max_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord , (((InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 12u32 => return InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 12u32 => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 12u32 , _) => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 12u32 , _) => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 12u32 => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 12u32 => InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_menuitem_79 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             13usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 6u32 , parent_index : 3u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 12u32 , parent_index : 5u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 6u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 13u32 , parent_index : 7u32 , item_array_index : 9u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_menuitem_79 , sp :: ItemVTable , sp :: AllowPin > ;
             10usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#root_54 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#root_38 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#background_layer_39 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_visibility_40 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#touch_area_41 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#rectangle_43 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_Opacity_47 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#shortcut_49 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#image_46 ()) , sp :: VOffset :: new (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#menuitem_79 () + InnerMenuItem_root_54 :: FIELD_OFFSETS . r#base_56 () + InnerMenuItemBase_root_38 :: FIELD_OFFSETS . r#label_48 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_menuitem_79) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_menuitem_79 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_menuitem_79 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_menuitem_79 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_menuitem_79 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_menuitem_79 {
         type Data = sp :: MenuEntry ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . set (_index as _) ;
             (InnerComponent_menuitem_79 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_menuitem_79 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_keybinding_81 {
         r#keybinding_81 : sp :: r#KeyBinding , r#model_data : sp :: Property < sp :: MenuEntry > , r#model_index : sp :: Property < i32 > , r#keybinding_81_height : sp :: Property < sp :: LogicalLength > , r#keybinding_81_min_height : sp :: Property < sp :: LogicalLength > , r#keybinding_81_min_width : sp :: Property < sp :: LogicalLength > , r#keybinding_81_preferred_height : sp :: Property < sp :: LogicalLength > , r#keybinding_81_preferred_width : sp :: Property < sp :: LogicalLength > , r#keybinding_81_width : sp :: Property < sp :: LogicalLength > , r#keybinding_81_x : sp :: Property < sp :: LogicalLength > , r#keybinding_81_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_keybinding_81 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_keybinding_81 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72_activated ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . call (& (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) . clone () as _ ,))) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) . r#enabled) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_preferred_height ()) . apply_pin (_self) . get () . get () . max ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_min_height ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) . r#shortcut) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_preferred_width ()) . apply_pin (_self) . get () . get () . max ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_min_width ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - ((((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - ((((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             (InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_keybinding_81 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_72 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_keybinding_81 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#keybinding_81 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_keybinding_81) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_keybinding_81 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_keybinding_81 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_keybinding_81 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_keybinding_81 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_keybinding_81 {
         type Data = sp :: MenuEntry ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             (InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . set (_index as _) ;
             (InnerComponent_keybinding_81 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_keybinding_81 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     impl InnerPopupMenuImpl_root_72 {
         fn new (globals : sp :: Rc < SharedGlobals >) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = globals ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             8usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 8u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 6u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerPopupMenuImpl_root_72 , sp :: ItemVTable , sp :: AllowPin > ;
             6usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#root_72 ()) , sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#focus_scope_74 ()) , sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#sub_menu_83 ()) , sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_shadow_75 ()) , sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_76 ()) , sp :: VOffset :: new (InnerPopupMenuImpl_root_72 :: FIELD_OFFSETS . r#frame_clip_77 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerPopupMenuImpl_root_72) ;
         }
     ;
     impl sp :: PinnedDrop for InnerPopupMenuImpl_root_72 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerPopupMenuImpl_root_72 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerPopupMenuImpl_root_72 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerPopupMenuImpl_root_72 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_57 : sp :: r#WindowItem , r#text_59 : sp :: r#SimpleText , r#empty_60 : sp :: r#Empty , r#rectangle_64 : sp :: r#Empty , r#empty_66 : sp :: r#Empty , r#empty_68 : sp :: r#Empty , r#rectangle_69 : sp :: r#BasicBorderRectangle , r#text_70 : sp :: r#SimpleText , r#lineedit_67 : InnerLineEdit_root_15 , r#button_71 : InnerButton_root_26 , r#root_57_connected : sp :: Property < bool > , r#root_57_empty_58_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_58_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_58_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_61_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_61_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_61_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_61_width : sp :: Property < sp :: LogicalLength > , r#root_57_empty_65_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_65_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_65_layout_organized_data : sp :: Property < sp :: SharedVector < u16 , > > , r#root_57_empty_65_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_65_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_66_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_66_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_66_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_68_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_57_empty_68_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_empty_68_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_57_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_57_rectangle_64_width : sp :: Property < sp :: LogicalLength > , r#root_57_status_text : sp :: Property < sp :: SharedString > , r#root_57_connect_clicked : sp :: Callback < (sp :: SharedString ,) , () > , r#root_57_disconnect_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_text_62 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMainWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMainWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                 }
            ) ;
             InnerLineEdit_root_15 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 8u32 - 1 , tree_index_of_first_child + 9u32 - 1) ;
             InnerButton_root_26 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 4u32 - 1 , tree_index_of_first_child + 20u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#background () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_v ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (20f64) . clone () as _ ;
                             the_struct . r#end = (20f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (16f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_h ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (20f64) . clone () as _ ;
                         the_struct . r#end = (20f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_layoutinfo_v ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_i_layout_29_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 16f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (20f64) . clone () as _ ;
                         the_struct . r#end = (20f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (16f64) . clone () as _ ;
                                 the_struct . r#end = (8f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (16f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_rectangle_64_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Horizontal as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Vertical as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_organized_data ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#organize_grid_layout (sp :: Slice :: from_slice (& [(sp :: GridLayoutInputData {
                         r#col : (65536f64) . clone () as _ , r#colspan : (1f64) . clone () as _ , r#new_row : (false) . clone () as _ , r#row : (65536f64) . clone () as _ , r#rowspan : (1f64) . clone () as _ , }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Horizontal as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_h ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((160f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_layout_17_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (12f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (12f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 19u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (12f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (12f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 19u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (12f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (12f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 19u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_7 = sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_57 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_7) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_7) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info_7) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_7) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = (320f64) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_7) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_8 = sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_57 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_8) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_8) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info_8) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_8) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = (280f64) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_8) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_rectangle_64_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_status_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Disconnected")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("VPN Demo")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#foreground () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((700f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("iOS VPN Demo")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("ip:port (e.g. 1.2.3.4:1080)")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: SharedString :: from ("127.0.0.1:1080")) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4283215696f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294198070f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#foreground () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_status_text ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get () {
                                 ({
                                     (InnerMainWindow :: FIELD_OFFSETS . r#root_57_disconnect_clicked ()) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connect_clicked ()) . apply_pin (_self) . call (& (((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) . clone () as _ ,)) }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [7usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_primary ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get () {
                         (sp :: SharedString :: from ("Disconnect")) as _ }
                     else {
                         sp :: SharedString :: from ("Connect VPN") }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_59 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_70 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_x ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_15 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (x)) ,) ;
             InnerButton_root_26 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 ..= 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 1u32 , order , visitor) }
                 3u32 ..= 5u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 3u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#root_57_layoutinfo_h ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = (320f64) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerMainWindow :: FIELD_OFFSETS . r#root_57_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = (280f64) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 ..= 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . subtree_range (dyn_index - 1u32) }
                 3u32 ..= 5u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . subtree_range (dyn_index - 3u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 ..= 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . subtree_component (dyn_index - 1u32 , subtree_index , result) }
                 3u32 ..= 5u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . subtree_component (dyn_index - 3u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [7usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [6usize]) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 7u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_h ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_v ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 8u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_65_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_66_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 18u32 => (((12f64) . clone ()) . clone () as sp :: Coord , ((12f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 19u32 => ((((((((((((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_58_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_68_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 9u32 ..= 17u32 => return InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . item_geometry (index - 9u32 + 1) , 20u32 ..= 26u32 => return InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . item_geometry (index - 20u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , 2u32 => sp :: r#AccessibleRole :: r#Groupbox , 4u32 => sp :: r#AccessibleRole :: r#Button , 8u32 => sp :: r#AccessibleRole :: r#TextInput , 19u32 => sp :: r#AccessibleRole :: r#Text , 8u32 => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessible_role (0) , 9u32 ..= 17u32 => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessible_role (index - 9u32 + 1) , 4u32 => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessible_role (0) , 20u32 ..= 26u32 => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessible_role (index - 20u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("iOS VPN Demo")) , (2u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (2u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("SOCKS5 Proxy Server")) , (4u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_text ()) . apply_pin (_self) . get ()) , (8u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_text ()) . apply_pin (_self) . get ()) , (8u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some ((InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (19u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerMainWindow :: FIELD_OFFSETS . r#root_57_status_text ()) . apply_pin (_self) . get ()) , (8u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessible_string_property (0 , what) , (9u32 ..= 17u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessible_string_property (index - 9u32 + 1 , what) , (4u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessible_string_property (0 , what) , (20u32 ..= 26u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessible_string_property (index - 20u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (4u32 , sp :: AccessibilityAction :: r#Default) => {
                     (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26_accessible_action_default ()) . apply_pin (_self) . call (& ()) }
                 (8u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15_accessible_action_set_value ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) }
                 (8u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessibility_action (0 , action) , (9u32 ..= 17u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . accessibility_action (index - 9u32 + 1 , action) , (4u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessibility_action (0 , action) , (20u32 ..= 26u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . accessibility_action (index - 20u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: SupportedAccessibilityAction :: r#Default , 8u32 => sp :: SupportedAccessibilityAction :: r#SetValue , 8u32 => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . supported_accessibility_actions (0) , 9u32 ..= 17u32 => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . supported_accessibility_actions (index - 9u32 + 1) , 4u32 => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . supported_accessibility_actions (0) , 20u32 ..= 26u32 => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . supported_accessibility_actions (index - 20u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 ..= 17u32 => InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () . apply_pin (_self) . item_element_infos (index - 9u32 + 1) , 20u32 ..= 26u32 => InnerMainWindow :: FIELD_OFFSETS . r#button_71 () . apply_pin (_self) . item_element_infos (index - 20u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_62 {
         r#text_62 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_62 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_62 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_85 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_85 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("SOCKS5 Proxy Server")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_57 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_57_empty_61_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("SOCKS5 Proxy Server")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_62 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_62 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_62 :: FIELD_OFFSETS . r#text_62 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_62) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_62 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_62 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_62 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_62 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_62 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_62 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             27usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 18u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 20u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 6u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 9u32 , parent_index : 7u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 10u32 , parent_index : 8u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 9u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 15u32 , parent_index : 10u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 14u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 14u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 16u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 3u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 20u32 , parent_index : 3u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 24u32 , parent_index : 4u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 4u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 4u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 20u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 20u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerMainWindow , sp :: ItemVTable , sp :: AllowPin > ;
             21usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#root_57 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_59 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#empty_60 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#empty_68 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#root_26 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_64 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#empty_66 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#root_15 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#background_16 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_1 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#focus_border_23 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#root_clip_2 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#placeholder_3 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_4 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_69 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_70 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#i_background_27 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#i_touch_area_34 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#i_focus_scope_35 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#button_71 () + InnerButton_root_26 :: FIELD_OFFSETS . r#i_border_28 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerMainWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerMainWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#MainWindow (sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_context (ctx : sp :: SlintContext) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_context (ctx) ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_connect_clicked (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connect_clicked ()) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_connect_clicked (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connect_clicked ()) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_connected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_connected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_connected ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_disconnect_clicked (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_disconnect_clicked ()) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_disconnect_clicked (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] (InnerMainWindow :: FIELD_OFFSETS . r#root_57_disconnect_clicked ()) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_socks5_address (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_socks5_address (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#lineedit_67 () + InnerLineEdit_root_15 :: FIELD_OFFSETS . r#base_18 () + InnerLineEditBase_root_1 :: FIELD_OFFSETS . r#text_input_5 () + sp :: r#TextInput :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_status_text (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_status_text ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_status_text (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_57_status_text ()) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#MainWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: StrongHandle for r#MainWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow > ;
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             sp :: WindowInner :: from_pub (self . window ()) . context () . run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_FluentPalette_85 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_85 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_FluentPalette_85 : InnerFluentPalette_85 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_FluentPalette_85 . clone () . init (& _self) ;
             _self }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         # [cfg (false)] fn create_window_from_context (& self , ctx : sp :: SlintContext) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = ctx . platform () . create_window_adapter () ? ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_context (ctx) ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"24\" height=\"24\" fill=\"none\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"m4.397 4.554.073-.084a.75.75 0 0 1 .976-.073l.084.073L12 10.939l6.47-6.47a.75.75 0 1 1 1.06 1.061L13.061 12l6.47 6.47a.75.75 0 0 1 .072.976l-.073.084a.75.75 0 0 1-.976.073l-.084-.073L12 13.061l-6.47 6.47a.75.75 0 0 1-1.06-1.061L10.939 12l-6.47-6.47a.75.75 0 0 1-.072-.976l.073-.084-.073.084Z\" fill=\"#212121\"/></svg>" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"24\" height=\"24\" fill=\"none\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M2.22 2.22a.75.75 0 0 0-.073.976l.073.084 4.034 4.035a9.986 9.986 0 0 0-3.955 5.75.75.75 0 0 0 1.455.364 8.49 8.49 0 0 1 3.58-5.034l1.81 1.81A4 4 0 0 0 14.8 15.86l5.919 5.92a.75.75 0 0 0 1.133-.977l-.073-.084-6.113-6.114.001-.002-1.2-1.198-2.87-2.87h.002L8.719 7.658l.001-.002-1.133-1.13L3.28 2.22a.75.75 0 0 0-1.06 0Zm7.984 9.045 3.535 3.536a2.5 2.5 0 0 1-3.535-3.535ZM12 5.5c-1 0-1.97.148-2.889.425l1.237 1.236a8.503 8.503 0 0 1 9.899 6.272.75.75 0 0 0 1.455-.363A10.003 10.003 0 0 0 12 5.5Zm.195 3.51 3.801 3.8a4.003 4.003 0 0 0-3.801-3.8Z\" fill=\"#212121\"/></svg>" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = b"<svg width=\"24\" height=\"24\" fill=\"none\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M12 9.005a4 4 0 1 1 0 8 4 4 0 0 1 0-8Zm0 1.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5ZM12 5.5c4.613 0 8.596 3.15 9.701 7.564a.75.75 0 1 1-1.455.365 8.503 8.503 0 0 0-16.493.004.75.75 0 0 1-1.455-.363A10.003 10.003 0 0 1 12 5.5Z\" fill=\"#212121\"/></svg>" ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" height=\"24px\" viewBox=\"0 -960 960 960\" width=\"24px\" fill=\"#e8eaed\">\n  <path d=\"m321-80-71-71 329-329-329-329 71-71 400 400L321-80Z\"/>\n</svg>\n" ;
     }
 # [allow (unused_imports)] pub use slint_generatedMainWindow :: {
     r#MainWindow , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
