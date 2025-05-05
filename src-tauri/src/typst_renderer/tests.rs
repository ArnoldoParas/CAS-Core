#[allow(unused_imports)]
use super::{generate_pdf, Data, LabelData, LabelStyle};

#[test]
fn label_test() {
    println!(
        "Directorio de trabajo actual: {:?}",
        std::env::current_dir().unwrap()
    );
    let data = LabelData {
        style: LabelStyle::Type1,
        dependence: "FIME".to_string(),
        amount: 20,
        start: 1,
    };

    generate_pdf(Data::Label(data)).unwrap();
}
