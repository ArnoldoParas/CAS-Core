mod environment;
mod tests;

use environment::TypstWrapperWorld;
use std::fs;
use typst::layout::Abs;
use typst_pdf::PdfOptions;

use crate::labels::generate_labels;

pub enum LabelStyle {
    Type1,
    Type2,
    CustomType(String),
}

enum Data {
    Maintenance(MaintenanceData),
    Label(LabelData),
}

pub struct LabelData {
    pub style: LabelStyle,
    pub dependence: String,
    pub amount: u16,
    pub start: u16,
}

pub struct MaintenanceData {
    dependence: Option<String>,
    head: Option<String>,
    department: Option<String>,
    identifiers: Vec<String>,
}

impl MaintenanceData {
    fn new(equipos: Vec<String>) -> Self {
        Self {
            dependence: None,
            head: None,
            department: None,
            identifiers: equipos,
        }
    }

    fn with_dependencia(mut self, dependencia: &str) -> Self {
        self.dependence = Some(dependencia.to_string());
        self
    }

    fn with_titular(mut self, titular: &str) -> Self {
        self.head = Some(titular.to_string());
        self
    }

    fn with_departamento(mut self, departamento: &str) -> Self {
        self.department = Some(departamento.to_string());
        self
    }
}

#[allow(private_interfaces)]
pub fn generate_pdf(data: Data) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("src/output/pdf")?;
    fs::create_dir_all("src/output/svg")?;

    let typst_content = match &data {
        Data::Maintenance(maintenance_data) => generate_typst_content(maintenance_data),
        Data::Label(label_data) => generate_labels(label_data).expect("Error generating labels"),
    };

    let world = TypstWrapperWorld::new("./".to_owned(), typst_content);

    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    match data {
        Data::Maintenance(_) => {
            let pdf =
                typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
            fs::write("src/output/pdf/output.pdf", pdf).expect("Error writing PDF.");
            println!("Created pdf: `src/output/pdf/output.pdf`");

            let svg = typst_svg::svg_merged(&document, Abs::pt(2.0));
            fs::write("src/output/svg/output.svg", svg).expect("Error writing SVG.");
            println!("Created svg: `src/output/svg/output.svg`");
        }
        Data::Label(_) => {
            let pdf =
                typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
            fs::write("src/output/pdf/output_label.pdf", pdf).expect("Error writing PDF.");
            println!("Created pdf: `src/output/pdf/output_label.pdf`");

            let svg = typst_svg::svg_merged(&document, Abs::pt(0.0));
            fs::write("src/output/svg/output_label.svg", svg).expect("Error writing SVG.");
            println!("Created svg: `src/output/svg/output_label.svg`");
            std::fs::remove_dir_all("src/assets/img/temp").ok();
        }
    }

    Ok(())
}

fn generate_typst_content(data: &MaintenanceData) -> String {
    let num_equipos = data.identifiers.len();
    let column_num = match num_equipos {
        0..=25 => 1,
        26..=50 => 2,
        51..=75 => 3,
        76..=100 => 4,
        _ => 4,
    };

    let column_config = format!("columns: ({})", vec!["1fr"; column_num].join(", "));

    let mut devices_per_column: Vec<Vec<(usize, &str)>> = vec![Vec::new(); column_num];
    for (i, device) in data.identifiers.iter().enumerate() {
        let column_idx = i / 25;
        if column_idx < column_num {
            devices_per_column[column_idx].push((i + 1, device));
        }
    }

    let content_columns = devices_per_column
        .iter()
        .map(|column| {
            let items = column
                .iter()
                .map(|(num, id)| format!("{}. {}", num, id))
                .collect::<Vec<String>>()
                .join("\n");
            format!("align(center)[\n{}\n]", items)
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let template = include_str!("./assets/templates/mantenimiento_preventivo_fime.typ");

    let content = template
        .replace("{{TOTAL_EQUIPOS}}", &num_equipos.to_string())
        .replace("{{COLUMN_CONFIG}}", &column_config)
        .replace("{{CONTENT_COLUMNS}}", &content_columns);

    content
}

pub fn generate_example_pdf() {
    let mut equipos = Vec::new();

    for _ in 0..81 {
        equipos.push("TEST >:D".to_string());
    }

    let data = MaintenanceData::new(equipos)
        .with_dependencia("FIME")
        .with_titular("Dr. Ejemplo")
        .with_departamento("Sistemas");

    if let Err(e) = generate_pdf(Data::Maintenance(data)) {
        eprintln!("Error generando PDF: {}", e);
    }
}
