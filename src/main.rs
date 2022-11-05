#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use serde_derive::{Deserialize, Serialize};
use csv::Reader;
use std::error::Error;
use std::collections::HashMap;
use std::str::FromStr;
use std::process;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AnzFile {
    pub details: String,
    pub particulars: String,
    pub code: String,
    pub reference: String,
    pub amount: String,
    pub date: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QbFile {
    pub name: String,
    pub split: String,
    pub date: String,
    pub amount: String
}
#[derive(Debug)]
pub struct AnzErrorMessage{
    pub amount: String,
    pub frequency: usize,
    pub qb_frequency: usize,
    pub qb_dates: Vec<String>,
    pub qb_names: Vec<String>,
    pub dates: Vec<String>,
    pub details: Vec<String>,
    pub particulars: Vec<String>,
    pub error_message: String 
}
#[derive(Debug)]
pub struct QbErrorMessage{
    pub amount: String,
    pub dates: String,
    pub names: String,
    pub anz_frequency: String,
    pub anz_dates: String,
    pub error_message: String 
}
#[derive(Debug)]
pub struct DoesntExistMessage{
    pub amount: String,
    pub dates: Vec<String>,
    pub names: Vec<String>,
    pub error_message: String
}

impl AsRef<String> for AnzFile {
    fn as_ref(&self) -> &String {
        &self.amount
    }

}

impl AsRef<String> for QbFile {
    fn as_ref(&self) -> &String {
        &self.amount
    }

}


fn main(){

    let mut anz_struct_data: Vec<AnzFile> = Vec::new();
    let mut qb_struct_data: Vec<QbFile> = Vec::new();

    let anz_file: String = "src/one_year_anz.csv".to_owned();
    let qb_file: String = "src/one_year_qb.csv".to_owned();

    match read_anz_file(&anz_file){
        Err(e) =>{
            eprintln!("{}", e);
        }
        Ok(v)=>{
            anz_struct_data = v;
        }
    }
    match read_qb_file(&qb_file){
        Err(e) =>{
            eprintln!("{}", e);
        }
        Ok(v)=>{
            qb_struct_data = v;
        }
    };   

    let mut anz_hash: HashMap<String, Vec<AnzFile>> = HashMap::new();
    let mut anz_hash_count: HashMap<String, i32> = HashMap::new();
    let mut qb_hash: HashMap<String, Vec<QbFile>> = HashMap::new();
    let mut qb_hash_count: HashMap<String, i32> = HashMap::new();
    
    let mut _v1 = check_and_return_array(anz_struct_data, anz_hash, anz_hash_count);

    println!("{:#?}", _v1);
    process::exit(1);
    for i in anz_struct_data{
        match anz_hash.get_mut(&i.amount){
            Some(v) =>{
                v.push(i.clone());
            }
            None =>{
                anz_hash.insert(i.amount.clone(), vec![i.clone()]);
            }
        }
        match anz_hash_count.get_mut(&i.amount){
            Some(v) =>{
                *v += 1;
            }
            None =>{
                anz_hash_count.insert(i.amount.clone(), 1);
            }
        }
    }

    for i in qb_struct_data{
        match qb_hash.get_mut(&i.amount){
            Some(v) =>{
                v.push(i.clone());
            }
            None =>{
                qb_hash.insert(i.amount.clone(), vec![i.clone()]);
            }
        }
        match qb_hash_count.get_mut(&i.amount){
            Some(v) =>{
                *v += 1;
            }
            None =>{
                qb_hash_count.insert(i.amount.clone(), 1);
            }
        }
    }
    // compare the qbfile has count to the anz

    let mut values_to_remove: Vec<String> = Vec::new();
    let mut qb_values_to_remove: Vec<String> = Vec::new();

    for (k1, v1) in qb_hash_count.iter(){
        //matches literals
        match anz_hash_count.get(&k1 as &str){
            Some(v) => {
                if v1 == v{
                    values_to_remove.push(k1.clone());
                    qb_values_to_remove.push(k1.clone());
                    continue;
                }
            }
            None => (),
        }

        //matches floats
        let f:f32 = k1.clone().parse::<f32>().unwrap();
        match anz_hash_count.get(&f.to_string() as &str){
            Some(v) => {
                if v1 == v{
                    values_to_remove.push(f.clone().to_string());
                    qb_values_to_remove.push(k1.clone());
                    continue;
                }
            }
            None => (),
        }

        //ANZ CSV takes the trailing 0's off decimal places... this catches that
        let mut trailing_zero: String = k1.to_string();
        trailing_zero.pop();

        match anz_hash_count.get(&trailing_zero as &str){
            Some(v) => {
                if v1 == v{
                    values_to_remove.push(trailing_zero.clone());
                    qb_values_to_remove.push(k1.clone());
                    continue;
                }
            }
            None => (),
        }

        //ANZ In all its wisom also removes decimals for whole numbers
        trailing_zero.pop();
        trailing_zero.pop();
        match anz_hash_count.get(&trailing_zero as &str){
            Some(v) => {
                if v1 == v{
                    values_to_remove.push(trailing_zero.clone());
                    qb_values_to_remove.push(k1.clone());
                    continue;
                }
            }
            None => (),
        }
    }


    for i in values_to_remove{
        match anz_hash_count.remove(&i as &str){
            Some(v) => {}
            None => (),
        }

        match qb_hash_count.remove(&i as &str){
            Some(v) => {}
            None => (),
        }
    }

    for i in qb_values_to_remove{
        match anz_hash_count.remove(&i as &str){
            Some(v) => {}
            None => (),
        }

        match qb_hash_count.remove(&i as &str){
            Some(v) => {}
            None => (),
        }
    }

    // At this point we only have the values that don't exist, entered incorrectly,
    // or don't show up as many times as expected. 

    // first lets look, if we have a corrosponding value that matches we can figure out
    // who we needs to investigate. 
    let mut doesnt_exist:Vec<String> = Vec::new();
    let mut investigate_anz: Vec<String> = Vec::new();
    let mut investigate_qb: Vec<String> = Vec::new();

    for (k1, v1) in &qb_hash_count{
        // to eliminate the fact it still could be decimal issue
        let mut trailing_zero = k1.clone().to_string();
        match anz_hash_count.get(&k1 as &str){
            Some(v) => {
                if v > &v1 {
                    investigate_qb.push(k1.clone());
                }
                else{
                    investigate_anz.push(k1.clone());
                }
                continue;
            }
            None => (),
        }

        trailing_zero.pop();
        match anz_hash_count.get(&trailing_zero as &str){
            Some(v) => {
                if v > &v1 {
                    investigate_qb.push(k1.clone());
                }
                else{
                    investigate_anz.push(k1.clone());
                }
                continue;
            }
            None => (),
        }

        trailing_zero.pop();
        trailing_zero.pop();

        match anz_hash_count.get(&trailing_zero as &str){
            Some(v) => {
                if v > &v1 {
                    investigate_qb.push(k1.clone());
                }
                else{
                    investigate_anz.push(k1.clone());
                }
                continue;
            }
            None => (),
        }

        doesnt_exist.push(k1.clone());
    }

    // investigate anz has values that appear more often in anz,
    // investigate qb has values that appear more often in qb,
    // doesnt_exist has values that only appear in one or the other
    // No we need to get matches,

    //dealing with no matches
    let mut doesnt_exist_error: Vec<DoesntExistMessage> = Vec::new();
    for i in &doesnt_exist{
        match qb_hash.get_key_value(i){
            Some((k,v)) => {
                let mut error_message:String = format!("The value {} exists in QUICKBOOKS but can't be found in ANZ", i.clone());
                let mut dates: Vec<String> = Vec::new(); 
                let mut names: Vec<String> = Vec::new(); 
                for mut x in v.clone() {
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    dates.push(x.clone().date);
                    names.push(x.clone().name);
                }
                let mut temp: DoesntExistMessage = DoesntExistMessage {
                    amount: i.clone(),
                    error_message: error_message,
                    dates: dates,
                    names: names
                };
                doesnt_exist_error.push(temp);
            },
            None => (),
        }
    }

    let mut anz_error: Vec<AnzErrorMessage> = Vec::new();
    for i in investigate_anz{
        match qb_hash.get_key_value(&i){
            Some((k,v)) => {
                let mut error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
                let mut dates: Vec<String> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                let mut particulars: Vec<String> = Vec::new();
                let mut details: Vec<String> = Vec::new();
                let mut qb_dates: Vec<String> = Vec::new();
                let mut qb_names: Vec<String> = Vec::new();
                let mut freq: usize = 0;


                //QUICKBOOKS DATA
                for mut x in v.clone(){
                    qb_dates.push(x.clone().date);
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    qb_names.push(x.clone().name);
                }

                //ANZ DATA
                // because of the whackness we may need to remove the trailing zeros to find
                let str1: String = remove_trailing_zeros(i.to_string());
                match anz_hash.get_key_value(&str1){
                    Some((k,v)) => {
                        for mut x in v.clone(){
                            dates.push(x.clone().date);
                            names.push(x.clone().details);
                            particulars.push(x.clone().particulars);
                            freq = v.len();
                        }
                    }
                    None => (),
                }

                let mut anz: AnzErrorMessage = AnzErrorMessage{
                    amount: i.clone(),
                    frequency: freq,
                    qb_frequency: v.clone().len(),
                    qb_dates: qb_dates,
                    qb_names: qb_names,
                    details: names,
                    dates: dates,
                    particulars: particulars,
                    error_message: error_message
                };

                anz_error.push(anz);
            }
            None => (),
        }
    }

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

    //for i in investigate_qb{
    //    match anz_hash.get_key_value(&i){
    //        Some((k,v)) => {
    //            let mut error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
    //            let mut dates: Vec<String> = Vec::new();
    //            let mut names: Vec<String> = Vec::new();
    //        }
    //        None => (),
    //    }
    //}

    for i in doesnt_exist_error{
        println!("{}", &i.error_message);
        println!("\nThese occur on dates:");
        for x in &i.dates{
            println!("{}", x);
        }
        println!("\nBy:");
        for x in &i.names{
            println!("{}", x);
        }
        println!("\n\n\n");
    }
//
//    println!("{:#?}", anz_hash_count);
//    println!("{:#?}", qb_hash_count);
//    println!("{:#?}", doesnt_exist);
//    println!("{:#?}", investigate_anz);
//    println!("{:#?}", investigate_qb);

}

fn check_and_return_array<T: AsRef<String> +Clone + std::fmt::Debug>
                        (struct_data: Vec<T>, mut hash_map: HashMap<String, Vec<T>>,
                         mut hash_counter: HashMap<String, i32>) 
                         -> (HashMap<String, Vec<T>>, HashMap<String, i32>){
    for i in struct_data{ 
        match hash_map.get_mut(&*i.as_ref()){
            Some(v) =>{
                v.push(i.clone());
            }
            None =>{
                hash_map.insert((*i.as_ref().clone()).to_string(), vec![i.clone()]);
            }
        }
    }
    (hash_map, hash_counter)
}

fn remove_trailing_zeros(mut number: String) -> String{

    if number.chars().last().unwrap() == '0' || number.chars().last().unwrap() == '.' {
        number.pop();
        number = remove_trailing_zeros(number.clone());
    }

    number.to_string()
}


fn read_anz_file(anz_file: &str)-> Result<Vec<AnzFile>, Box<dyn Error>>{
    let mut rdr = Reader::from_path(anz_file)
    .expect("COULDNT READ anz_file");
    let iter = rdr.deserialize();
    let mut anz_struct_vec: Vec<AnzFile> = Vec::new();

    for result in iter{
       anz_struct_vec.push(result?);
    }
    Ok(anz_struct_vec)
}

fn read_qb_file(qb_file: &str)-> Result<Vec<QbFile>, Box<dyn Error>>{
    let mut rdr = Reader::from_path(qb_file)
    .expect("COULDNT READ anz_file");
    let iter = rdr.deserialize();
    let mut struct_vec: Vec<QbFile> = Vec::new();

    for result in iter{
        struct_vec.push(result?);
    }
    Ok(struct_vec)
}
