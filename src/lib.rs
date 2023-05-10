use std::ops::Range;

use ariadne::{Label, Report, ReportKind, Source};

#[derive(Clone, Debug)]
pub struct RosyError {
	pub title: String,
	pub description: String,
	pub labels: Vec<(String, Range<usize>)>,
}

pub fn report(err: RosyError, source: &str, filename: &str) {
	let RosyError {
		title,
		description,
		labels,
	} = err.into();

	let location = &labels[0].1;

	let mut builder =
		Report::build(ReportKind::Error, filename, location.start).with_message(title);

	for label in labels {
		let (msg, span) = label;
		builder.add_label(Label::new((filename, span)).with_message(msg));
	}

	builder
		.with_note(description)
		.finish()
		.eprint((filename, Source::from(source)))
		.unwrap();
}
