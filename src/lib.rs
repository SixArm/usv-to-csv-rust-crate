use usv::*;
use std::convert::AsRef;

pub fn usv_to_csv<
    S: AsRef<str> + Sized
>(
    usv: S
) -> String {
    usv_to_csv_with_separators(usv, ",", "\n")
}

pub fn usv_to_csv_with_separators<
    S0: AsRef<str> + Sized, 
    S1: AsRef<str> + Sized, 
    S2: AsRef<str> + Sized,
>(
    usv: S0,
    output_unit_separator: S1,
    output_record_separator: S2,
) -> String {
    let mut record_separating = false;
    let mut unit_separating = false;
    let mut s = String::new();
    let records: Records = usv.as_ref().records().collect();
    for record in records {
        if record_separating {
            s +=  output_record_separator.as_ref()
        } else {
            record_separating = true;
        }
        unit_separating = false;
        for unit in record {
            if unit_separating {
                s += output_unit_separator.as_ref()
            } else {
                unit_separating = true;
            }
            if unit.contains("\"")
            || unit.contains(output_unit_separator.as_ref())
            || unit.contains(output_record_separator.as_ref()) {
                s += &format!("\"{}\"", unit.replace("\"", "\"\""));
            } else {
                s += &unit;
            };
        };
    };
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let usv = String::from("a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞");
        let csv = String::from("a,b,c\nd,e,f\ng,h,i");
        assert_eq!(usv_to_csv(&usv), csv);
    }

    #[test]
    fn quotes() {
        let usv = String::from("a\"b\"c␟␞");
        let csv = String::from("\"a\"\"b\"\"c\"");
        assert_eq!(usv_to_csv(&usv), csv);
    }

    #[test]
    fn commas() {
        let usv = String::from("a,b,c␟␞");
        let csv = String::from("\"a,b,c\"");
        assert_eq!(usv_to_csv(&usv), csv);
    }

    #[test]
    fn with_output_separators() {
        let usv = String::from("a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞");
        let csv = String::from("a;b;c!d;e;f!g;h;i");
        assert_eq!(usv_to_csv_with_separators(&usv, ";", "!"), csv);
    }

}
