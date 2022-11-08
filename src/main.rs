#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod print;

use serde_derive::Deserialize;
use csv::Reader;
use std::error::Error;
use std::collections::HashMap;
use std::process;

use crate::print::print::print_anz;
use crate::print::print::print_qb;
use crate::print::print::print_doesnt_exist;

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct QbErrorMessage{
    pub amount: String,
    pub dates: Vec<String>,
    pub names: Vec<String>,
    pub frequency: usize,
    pub anz_frequency: String,
    pub anz_dates: Vec<String>,
    pub anz_names: Vec<String>,
    pub error_message: String 
}
#[derive(Debug, Clone)]
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
    fn as_ref_date(&self) -> &String{
        &self.date
    }
}

impl AsRef<String> for QbFile {
    fn as_ref(&self) -> &String {
        &self.amount
    }
    fn as_ref_date(&self) -> &String{
        &self.date
    }
}

pub trait AsRef<String>{
    fn as_ref(&self) -> &String;
    fn as_ref_date(&self) -> &String;
}


fn main(){

    let mut anz_struct_data: Vec<AnzFile> = Vec::new();
    let mut qb_struct_data: Vec<QbFile> = Vec::new();

    let anz_file: String = "src/one_year_anz.csv".to_owned();
    let qb_file: String = "src/one_year_qb.csv".to_owned();

    match read_csv_return_struct(&anz_file){
        Err(e) => eprintln!("{}", e),
        Ok(v)=> anz_struct_data = v,
    }

    match read_csv_return_struct(&qb_file){
        Err(e) => eprintln!("{}", e),
        Ok(v)=> qb_struct_data = v,
    }
    let mut anz_hash: HashMap<String, Vec<AnzFile>> = HashMap::new();
    let mut anz_hash_count: HashMap<String, i32> = HashMap::new();
    let mut qb_hash: HashMap<String, Vec<QbFile>> = HashMap::new();
    let mut qb_hash_count: HashMap<String, i32> = HashMap::new();
    let mut anz_values_to_remove: Vec<String> = Vec::new();
    let mut qb_values_to_remove: Vec<String> = Vec::new();

    check_and_prune_structs(&mut anz_struct_data, &mut qb_struct_data);
    check_and_return_hash(anz_struct_data.clone(), &mut anz_hash, &mut anz_hash_count);
    check_and_return_hash(qb_struct_data.clone(), &mut qb_hash, &mut qb_hash_count);
    
    for (k1, v1) in qb_hash_count.iter(){
        // matches with the leading 0 removed
        let without_trailing_zero = remove_trailing_zeros(k1.clone().to_string(), 0);
        get_matching_values(anz_hash_count.clone(), &mut anz_values_to_remove, &mut qb_values_to_remove, without_trailing_zero.clone(), k1.to_string(), v1);
        
    }

    remove_matching_values_from_counter(anz_values_to_remove.clone(), &mut anz_hash_count, &mut qb_hash_count);
    remove_matching_values_from_counter(qb_values_to_remove.clone(), &mut anz_hash_count, &mut qb_hash_count);

    // At this point we only have the values that don't exist, entered incorrectly,
    // or don't show up as many times as expected. 

    // first lets look, if we have a corrosponding value that matches we can figure out
    // who we needs to investigate. 
    let mut doesnt_exist_in_anz:Vec<String> = Vec::new();
    let mut doesnt_exist_in_qb:Vec<String> = Vec::new();
    let mut investigate_anz: Vec<String> = Vec::new();
    let mut investigate_qb: Vec<String> = Vec::new();

    for (key, value) in &qb_hash_count{
        // to eliminate the fact it still could be decimal issue
        let mut exists = false;
        build_investigation(anz_hash_count.clone(), key, key, value, &mut exists, &mut investigate_qb, &mut investigate_anz);
        if !exists {
            let trailing_zero = remove_trailing_zeros(key.clone().to_string(), 0);
            build_investigation(anz_hash_count.clone(), &trailing_zero, key, value, &mut exists, &mut investigate_qb, &mut investigate_anz);
        }
        if !exists {
            doesnt_exist_in_anz.push(key.clone());
        }
    }

    for (k1, v1) in &anz_hash_count{
        // to eliminate the fact it still could be decimal issue
        let mut exists = false;
        for (k2,v2) in &qb_hash_count {
            match remove_trailing_zeros(k1.clone().to_string(), 0) == remove_trailing_zeros(k2.clone().to_string(), 0){
                true => {
                    exists = true;
                }
                false => (),
            }
        }
        if ! exists{
            doesnt_exist_in_qb.push(k1.clone());
        }
    }

    // investigate anz has values that appear more often in anz,
    // investigate qb has values that appear more often in qb,
    // doesnt_exist has values that only appear in one or the other
    // No we need to get matches,

    //dealing with no matches
    let mut doesnt_exist_error: Vec<DoesntExistMessage> = Vec::new();
    for i in &doesnt_exist_in_anz{
        match qb_hash.get_key_value(i){
            Some((k,v)) => {
                let error_message:String = format!("The value {} exists in QUICKBOOKS but can't be found in ANZ", i.clone());
                let mut dates: Vec<String> = Vec::new(); 
                let mut names: Vec<String> = Vec::new(); 
                for mut x in v.clone() {
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    dates.push(x.clone().date);
                    names.push(x.clone().name);
                }
                let temp: DoesntExistMessage = DoesntExistMessage {
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

    for i in &doesnt_exist_in_qb{
        match anz_hash.get_key_value(i){
            Some((k,v)) => {
                let error_message:String = format!("The value {} exists in ANZ but can't be found in QUICKBOOKS", i.clone());
                let mut dates: Vec<String> = Vec::new(); 
                let mut names: Vec<String> = Vec::new();
                for x in v.clone() {
                    dates.push(x.clone().date);
                    names.push(x.clone().details);
                }
                let temp: DoesntExistMessage = DoesntExistMessage {
                    amount: i.clone(),
                    error_message: error_message,
                    dates: dates,
                    names: names
                };
                doesnt_exist_error.push(temp);
            },
            None=>(),
        }
    }

    let mut anz_error: Vec<AnzErrorMessage> = Vec::new();
    for i in investigate_anz{
        match qb_hash.get_key_value(&i){
            Some((k,v)) => {
                let error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
                let mut dates: Vec<String> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                let mut particulars: Vec<String> = Vec::new();
                let mut qb_dates: Vec<String> = Vec::new();
                let mut qb_names: Vec<String> = Vec::new();
                let mut freq: usize = 0;

                for mut x in v.clone(){
                    qb_dates.push(x.clone().date);
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    qb_names.push(x.clone().name);
                }

                let str1: String = remove_trailing_zeros(i.to_string(), 0);
                match anz_hash.get_key_value(&str1){
                    Some((k,v)) => {
                        for x in v.clone(){
                            dates.push(x.clone().date);
                            names.push(x.clone().details);
                            particulars.push(x.clone().particulars);
                            freq = v.len();
                        }
                    }
                    None => (),
                }

                let anz: AnzErrorMessage = AnzErrorMessage{
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

    let mut qb_error: Vec<QbErrorMessage> = Vec::new();
    for i in investigate_qb{
        match anz_hash.get_key_value(&i){
            Some((k,v)) => {
                let error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
                let mut dates: Vec<String> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                let mut anz_names: Vec<String> = Vec::new();
                let mut anz_dates: Vec<String> = Vec::new();

                for x in v.clone(){
                    anz_names.push(x.clone().details);
                    anz_dates.push(x.clone().date);
                }
                match qb_hash.get_key_value(&i){
                    Some((k,v)) => {
                        for x in v.clone(){
                            dates.push(x.clone().date);
                            names.push(x.clone().name);
                        }
                    }
                    None => (),
                }
                let qb: QbErrorMessage = QbErrorMessage{
                    amount: i.clone(),
                    dates: dates,
                    names: names,
                    frequency: v.len(),
                    anz_frequency: v.len().to_string(),
                    anz_dates: anz_dates, 
                    anz_names: anz_names,
                    error_message: error_message
                };

                qb_error.push(qb);
            }
            None => (),
        }
    }
    let mut debug = false;
    if !debug {
        print_anz(anz_error);
        print_qb(qb_error);
        print_doesnt_exist(doesnt_exist_error);
    }

}

fn check_and_prune_structs<T: AsRef<String> + Clone + std::fmt::Debug, U: AsRef<String> + Clone + std::fmt::Debug>
                          (anz_struct: &mut Vec<T>, qb_struct: &mut Vec<U>){
    let mut anz_index_to_remove: Vec<i32> = Vec::new();                     
    let mut qb_index_to_remove: Vec<i32> = Vec::new();

    for (pos, i) in anz_struct.iter().enumerate(){
        for (pos2, i2) in qb_struct.iter().enumerate(){
            let qb_value = remove_trailing_zeros((&i2.as_ref()).to_string(), 0);
            if (&i.as_ref()).to_string() == qb_value{
                if &i.as_ref_date() == &i2.as_ref_date(){
                    if ! anz_index_to_remove.contains(&(pos as i32)){
                        anz_index_to_remove.push(pos as i32);
                    }
                    if !qb_index_to_remove.contains(&(pos2 as i32)){
                        qb_index_to_remove.push(pos2 as i32);
                    }
                }
            }
        }
    }
    let mut anz_removed:i32 = 0; 
    let mut qb_removed:i32 = 0; 
    for mut i in anz_index_to_remove{
        i -= anz_removed;
        let g = anz_struct.remove(i as usize);
        anz_removed += 1;
    }
    qb_index_to_remove.sort();
    for mut x in qb_index_to_remove{
        x -= qb_removed;
        let g = qb_struct.remove(x as usize);
        qb_removed += 1;
    }

}

fn check_and_return_hash<T: AsRef<String> + Clone>
                        (struct_data: Vec<T>, hash_map: &mut HashMap<String, Vec<T>>,
                         hash_counter: &mut HashMap<String, i32>){
    for i in struct_data{ 
        match hash_map.get_mut(&*i.as_ref()){
            Some(v) =>{
                v.push(i.clone());
            }
            None =>{
                hash_map.insert((*i.as_ref().clone()).to_string(), vec![i.clone()]);
            }
        }
        match hash_counter.get_mut(&*i.as_ref()){
            Some(v) =>{
                *v += 1;
            }
            None =>{
                hash_counter.insert((*i.as_ref().clone()).to_string(), 1);
            }
        }
    }
}

fn get_matching_values(comp_hash_count: HashMap<String, i32>, anz_remove: &mut Vec<String>, qb_remove: &mut Vec<String>, 
                       key:String, original_key: String, value: &i32){
    
    match comp_hash_count.get(&key as &str){
        Some(v) => {
            if value == v{
                anz_remove.push(key.clone());
                qb_remove.push(original_key.clone());
            }
        }
        None => (),
    }
}

fn remove_matching_values_from_counter(values_to_remove:Vec<String>, hash_counter_one:&mut HashMap<String, i32>, hash_counter_two:&mut HashMap<String, i32>){
    for i in values_to_remove{
        hash_counter_one.remove(&i as &str);
        hash_counter_two.remove(&i as &str);
    }
}

fn build_investigation(hash_counter:HashMap<String, i32>, key: &String, original_key:&String, value: &i32, exists: &mut bool, qb_vec:&mut Vec<String>, anz_vec:&mut Vec<String>){
    match hash_counter.get(key as &str){
        Some(v) => {
            if v > value {
                qb_vec.push(original_key.clone());
            }
            else{
                anz_vec.push(key.clone());
            }
            *exists = true;
        }
        None => (),
    } 
}

fn remove_trailing_zeros(mut number: String, iterations: i32) -> String{
    if iterations >= 3{
        return number.to_string();
    } 
    if number.chars().last().unwrap() == '0' || number.chars().last().unwrap() == '.' {
        number.pop();
        number = remove_trailing_zeros(number.clone(), iterations + 1);
    }
    number.to_string()
}

fn read_csv_return_struct<T: for<'de> serde::Deserialize<'de>>(csv_file: &str) -> Result<Vec<T>, Box<dyn Error>>{
    let mut rdr = Reader::from_path(csv_file)
    .expect("COULDNT READ anz_file");
    let iter = rdr.deserialize();
    let mut csv_file_struct: Vec<T> = Vec::new();

    for result in iter{
        csv_file_struct.push(result?);
    }
    Ok(csv_file_struct)
}
