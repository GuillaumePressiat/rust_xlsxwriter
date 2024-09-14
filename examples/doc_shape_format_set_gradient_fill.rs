// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! This example demonstrates adding a Textbox shape and setting some of its
//! properties.

use rust_xlsxwriter::{
    Shape, ShapeFormat, ShapeGradientFill, ShapeGradientStop, Workbook, XlsxError,
};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Create a textbox shape with formatting.
    let textbox = Shape::textbox().set_text("This is some text").set_format(
        &ShapeFormat::new().set_gradient_fill(&ShapeGradientFill::new().set_gradient_stops(&[
            ShapeGradientStop::new("#F1DCDB", 0),
            ShapeGradientStop::new("#963735", 100),
        ])),
    );

    // Insert a textbox in a cell.
    worksheet.insert_shape(1, 1, &textbox)?;

    // Save the file to disk.
    workbook.save("shape.xlsx")?;

    Ok(())
}
