// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates setting different formatting for numbers
//! in an Excel worksheet.

use rust_xlsxwriter::{Format, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new("numbers.xlsx");

    // Create some formats to use with the numbers below.
    let number_format = Format::new()
        .set_num_format("#,##0.00")
        .register_with(&mut workbook);

    let currency_format = Format::new()
        .set_num_format("€#,##0.00")
        .register_with(&mut workbook);

    let percentage_format = Format::new()
        .set_num_format("0.0%")
        .register_with(&mut workbook);

    let bold_italic_format = Format::new()
        .set_bold()
        .set_italic()
        .register_with(&mut workbook);

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    worksheet.write_number(0, 0, 1234.5, &number_format)?;
    worksheet.write_number(1, 0, 1234.5, &currency_format)?;
    worksheet.write_number(2, 0, 0.3300, &percentage_format)?;
    worksheet.write_number(3, 0, 1234.5, &bold_italic_format)?;

    workbook.close()?;

    Ok(())
}