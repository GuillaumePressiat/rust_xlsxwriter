// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Shape, ShapeText, ShapeTextVerticalAlignment, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::Top),
        );

    worksheet.insert_shape(8, 4, &textbox)?;

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::Middle),
        );

    worksheet.insert_shape(18, 4, &textbox)?;

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::Bottom),
        );

    worksheet.insert_shape(28, 4, &textbox)?;

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::TopCentered),
        );

    worksheet.insert_shape(38, 4, &textbox)?;

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::MiddleCentered),
        );

    worksheet.insert_shape(48, 4, &textbox)?;

    let textbox = Shape::textbox()
        .set_text("This is some text")
        .set_text_options(
            &ShapeText::new().set_vertical_alignment(ShapeTextVerticalAlignment::BottomCentered),
        );

    worksheet.insert_shape(58, 4, &textbox)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_textbox18() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox18")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
