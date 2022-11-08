pub mod print{

    use crate::AnzErrorMessage;
    use crate::QbErrorMessage;
    use crate::DoesntExistMessage;

    use std::{fs::File, error::Error};
    use std::io::Write;

    pub fn print_anz(anz_error: Vec<AnzErrorMessage>) -> std::io::Result<()>{
        let mut f = File::create("anz_errors.txt").expect("Unable to create file");
        for i in anz_error{
            writeln!(f,"{} \nthis value appears {} times in ANZ vs {} times in QUICKBOOKS",
            &i.error_message, &i.frequency, &i.qb_frequency)?;
            writeln!(f,"\nANZ details are as follows: ")?;
            writeln!(f,"\nDates: ")?;
            for x in &i.dates{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nDetails: ")?;
            for x in &i.details{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nQUICKBOOKS details are as follows:")?;
            writeln!(f,"\nDates: ")?;
            for x in &i.qb_dates{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nNames: ")?;
            for x in &i.qb_names{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\n\n")?;

        }
        Ok(())
    }
    
    pub fn print_qb(qb_error: Vec<QbErrorMessage>) -> std::io::Result<()>{
        let mut f = File::create("quickbooks_errors.txt").expect("Unable to create file");
        for i in qb_error{
            writeln!(f,"{} \nthis value appears {} times in QUICKBOOKS vs {} times in ANZ",
            &i.error_message, &i.frequency, &i.anz_frequency)?;
            writeln!(f,"\nQUICKBOOK details are as follows: ")?;
            writeln!(f,"\nDates: ")?;
            for x in &i.dates{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nDetails: ")?;
            for x in &i.names{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nANZ details are as follows:")?;
            writeln!(f,"\nDates: ")?;
            for x in &i.anz_dates{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nNames: ")?;
            for x in &i.anz_names{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\n\n")?;
        }

        Ok(())
    }

    pub fn print_doesnt_exist(error: Vec<DoesntExistMessage>) -> std::io::Result<()>{
        let mut f = File::create("errors.txt").expect("Unable to create file");
        for i in error{
            writeln!(f,"{}", &i.error_message)?;
            writeln!(f,"\nThese occur on dates:")?;
            for x in &i.dates{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\nBy:")?;
            for x in &i.names{
                writeln!(f,"{}", x)?;
            }
            writeln!(f,"\n\n")?;
        }
        Ok(())
    }
}