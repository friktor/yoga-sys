extern crate bindgen;
extern crate git2;
extern crate gcc;

use bindgen::RustTarget;
use std::path::PathBuf;
use git2::Repository;
use gcc::Build;
use std::env;

fn update_submodules() -> Result<(), git2::Error> {
  let repo = Repository::open("./")?;
  let mut submodules = repo.submodules()?;
  
  for submodule in submodules.iter_mut() {
    submodule.update(true, None)?;
  }

  Ok(())
}

fn compile_yoga() {
  let mut c = Build::new();
  c.cpp(true);

  c.flag("-std=c++14");
  c.flag("-fno-omit-frame-pointer");
  c.flag("-fexceptions");
  c.flag("-Wall");
  c.flag("-Werror");
  c.flag("-O3");
  c.flag("-ffast-math");
  
  c.file("yoga/yoga/Yoga.cpp");
  c.compile("libyoga.a");
}

fn bindgen_yoga() {
	let bindings = bindgen::Builder::default()
		.rust_target(RustTarget::Nightly);
	
	let enumerated_and_edges_setters = vec![
		"Direction",
		"FlexDirection",
		"JustifyContent",
		"AlignContent",
		"AlignItems",
		"AlignSelf",
		"PositionType",
		"FlexWrap",
		"Overflow",
		"Display",
		"Flex",
		"FlexGrow",
		"FlexShrink",
		"AspectRatio",

		// Edges
		"Border"
	];

	let unit_and_edge_setters = vec![
		"MinWidth",
		"MinHeight",
		"MaxWidth",
		"MaxHeight",

		// Edge
		"Position",
		"Margin",
		"Padding",
	]

	let auto_unit_and_edge_setters = vec![
		"FlexBasis",
		"Width",
		"Height",

		// Edge
		"Margin",
	];

	for property in enumerated_setters {
		bindings.whitelist_function(format!("YGNodeStyleSet{}", property));
	}

	for property in unit_setters {
		bindings.whitelist_function(format!("YGNodeStyleSet{}", property));
		bindings.whitelist_function(format!("YGNodeStyleSet{}Percent", property));
	}

	for property in auto_unit_setters {
		bindings.whitelist_function(format!("YGNodeStyleSet{}", property));
		bindings.whitelist_function(format!("YGNodeStyleSet{}Percent", property));
		bindings.whitelist_function(format!("YGNodeStyleSet{}Auto", property));
	}

	bindings	
		// Basic props
		.whitelist_type("YGSize")
		.whitelist_type("YGConfig")
		.whitelist_type("YGValue")
		.whitelist_type("YGUnit")
		.whitelist_type("YGNode")

		// Getter calculated layout result
// Left
// Top
// Right
// Bottom
// Width
// Height
// Direction
// HadOverflow

		// Node
		.whitelist_function("YGNodeNew")
		.whitelist_function("YGNodeNewWithConfig")
		.whitelist_function("YGNodeClone")
		
		// Node drops
		.whitelist_function("YGNodeFree")
		.whitelist_function("YGNodeFreeRecursive")
		.whitelist_function("YGNodeReset")
		
		// Tree handlers
		.whitelist_function("YGNodeGetInstanceCount")
		.whitelist_function("YGNodeInsertChild")
		.whitelist_function("YGNodeInsertSharedChild")
		.whitelist_function("YGNodeRemoveChild")
		.whitelist_function("YGNodeRemoveAllChildren")
		.whitelist_function("YGNodeGetChild")
		.whitelist_function("YGNodeGetOwner")
		.whitelist_function("YGNodeGetChildCount")
		.whitelist_function("YGNodeSetChildren")
		.whitelist_function("YGNodeMarkDirty")
		.whitelist_function("YGNodeMarkDirtyAndPropogateToDescendants")
		
		// Node utils and other methods
		.whitelist_function("YGNodeCalculateLayout")
		.whitelist_function("YGNodeCanUseCachedMeasurement")
		.whitelist_function("YGFloatIsUndefined")
		.whitelist_function("YGNodeCopyStyle")
		.whitelist_function("YGNodePrint")
		
		// Context
		.whitelist_function("YGNodeGetContext")
		.whitelist_function("YGNodeSetContext")
		
		// Measure & Layout handlers
		.whitelist_function("YGNodeGetMeasureFunc")
		.whitelist_function("YGNodeSetMeasureFunc")
		
		.whitelist_function("YGNodeGetBaselineFunc")
		.whitelist_function("YGNodeSetBaselineFunc")
		
		.whitelist_function("YGNodeGetDirtiedFunc")
		.whitelist_function("YGNodeSetDirtiedFunc")
		
		.whitelist_function("YGNodeGetPrintFunc")
		.whitelist_function("YGNodeSetPrintFunc")

		.whitelist_function("YGNodeGetHasNewLayout")
		.whitelist_function("YGNodeSetHasNewLayout")
		
		// Node type
		.whitelist_function("YGNodeGetNodeType")
		.whitelist_function("YGNodeSetNodeType")
		
		// Checkers
		.whitelist_function("YGNodeIsDirty")
		.whitelist_function("YGNodeLayoutGetDidUseLegacyFlag")

		// Config and loggers
		.whitelist_function("YGLog")
		.whitelist_function("YGLogWithConfig")
		
		// Base Config
		.whitelist_function("YGConfigNew")
		.whitelist_function("YGConfigSetLogger")
		.whitelist_function("YGConfigFree")
		.whitelist_function("YGConfigCopy")
		
		// Config Layout
		.whitelist_function("YGConfigSetPointScaleFactor")
		.whitelist_function("YGConfigSetShouldDiffLayoutWithoutLegacyStretchBehaviour")
		.whitelist_function("YGConfigSetUseLegacyStretchBehaviour")
		.whitelist_function("YGConfigGetInstanceCount")

		// Enable Experemental Features
		.whitelist_function("YGConfigSetExperimentalFeatureEnabled")
		.whitelist_function("YGConfigIsExperimentalFeatureEnabled")
		
		// Assertions
		.whitelist_function("YGAssert")
		.whitelist_function("YGAssertWithNode")
		.whitelist_function("YGAssertWithConfig")

		.rustfmt_bindings(true)
		.rustified_enum("YG.*")
		.header("yoga/yoga/Yoga.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}

fn main() {
  update_submodules();
  compile_yoga();
  bindgen_yoga();
}