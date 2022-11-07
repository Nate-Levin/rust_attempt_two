pub mod print{

    use crate::AnzErrorMessage;
    use crate::QbErrorMessage;
    use crate::DoesntExistMessage;

    pub fn print_anz(anz_error: Vec<AnzErrorMessage>){
        for i in anz_error{
            println!("{} \nthis value appears {} times in ANZ vs {} times in QUICKBOOKS",
            &i.error_message, &i.frequency, &i.qb_frequency);
            println!("\nANZ details are as follows: ");
            println!("\nDates: ");
            for x in &i.dates{
                println!("{}", x);
            }
            println!("\nDetails: ");
            for x in &i.details{
                println!("{}", x);
            }
            println!("\nQUICKBOOKS details are as follows:");
            println!("\nDates: ");
            for x in &i.qb_dates{
                println!("{}", x);
            }
            println!("\nNames: ");
            for x in &i.qb_names{
                println!("{}", x);
            }
            println!("\n\n");
        }
    }
    
    pub fn print_qb(qb_error: Vec<QbErrorMessage>){
        for i in qb_error{
            println!("{} \nthis value appears {} times in QUICKBOOKS vs {} times in ANZ",
            &i.error_message, &i.frequency, &i.anz_frequency);
            println!("\nQUICKBOOK details are as follows: ");
            println!("\nDates: ");
            for x in &i.dates{
                println!("{}", x);
            }
            println!("\nDetails: ");
            for x in &i.names{
                println!("{}", x);
            }
            println!("\nANZ details are as follows:");
            println!("\nDates: ");
            for x in &i.anz_dates{
                println!("{}", x);
            }
            println!("\nNames: ");
            for x in &i.anz_names{
                println!("{}", x);
            }
            println!("\n\n");
        }
    }

    pub fn print_doesnt_exist(error: Vec<DoesntExistMessage>){
        for i in error{
            println!("{}", &i.error_message);
            println!("\nThese occur on dates:");
            for x in &i.dates{
                println!("{}", x);
            }
            println!("\nBy:");
            for x in &i.names{
                println!("{}", x);
            }
            println!("\n\n");
        }
    }
}