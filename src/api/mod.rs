use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use std::fs;
use serde_json;

mod models;

const PATH_TO_FILE:&str = "test.json";

fn get_all_receipts() -> Option<Vec<models::Receipt>> {
    let file_contents = fs::read_to_string(PATH_TO_FILE);
    match file_contents {
        Err(e) => {
            println!("{}", e);
            return None;
        },
        Ok(contents) => {
            let json: Result<Vec<models::Receipt>, serde_json::Error> = serde_json::from_str(&contents);
            match json {
                Err(e) => {
                    println!("{}", e);
                    return None;
                },
                Ok(receipts) => Some(receipts)
            }
        }
    }
}


fn write_receipt_file(receipts:&Vec<models::Receipt>) -> Result<(), std::io::Error> {
    let contents = serde_json::to_string(receipts).unwrap();
    fs::write(PATH_TO_FILE, contents)?;
    Ok(())
}


fn create_id(all_receipts:&Vec<models::Receipt>) -> u64 {
    
    let mut max_id = 1;
    for receipt in all_receipts {
        if receipt.id > max_id {
            max_id = receipt.id
        }
    }
    max_id += 1;
    max_id
}


#[get("/")]
async fn get_all() -> impl Responder {
    let receipts = get_all_receipts();
    match receipts {
        Some(r) => HttpResponse::Ok().json(r),
        None => HttpResponse::Ok().json(models::ErrorResponse::new("No receipts found."))
    }
}


#[get("/{receipt_id}")]
async fn get_by_id(receipt_id: web::Path<u64>) -> impl Responder {
    let has_receipts = get_all_receipts();
    match has_receipts {
        None => HttpResponse::Ok().json(models::ErrorResponse::new("No receipts found")),
        Some(receipts) => {
            let receipt_id = receipt_id.into_inner();
            let receipt: Option<&models::Receipt> = receipts
                .iter()
                .filter(|r| r.id == receipt_id)
                .nth(0);
            
            match receipt {
                None => HttpResponse::Ok().json(models::ErrorResponse::new(&format!("No receipt with id {} found", receipt_id))),
                Some(r) => HttpResponse::Ok().json(&r)
            }
        },
    }
}


#[post("/")]
async fn create(new_receipt: web::Json<models::ReceiptEntry>) -> impl Responder {
    let new_receipt = new_receipt.into_inner();
    let error_response = HttpResponse::Ok().json(models::ErrorResponse::new("No receipts found"));
    match get_all_receipts() {
        None => {
            error_response
        },
        Some(mut all_receipts) => {
            let max_id = create_id(&all_receipts);
            let new_receipt = models::Receipt {
                id: max_id,
                store: new_receipt.store,
                date: new_receipt.date,
                paid_by: new_receipt.paid_by,
                items: new_receipt.items,
                subtotal: new_receipt.subtotal,
                contributor_to_pay: new_receipt.contributor_to_pay,
            };
            all_receipts.push(new_receipt.clone());
            match write_receipt_file(&all_receipts) {
                Ok(_) => HttpResponse::Ok().json(new_receipt),
                Err(e) => HttpResponse::Ok().json(models::ErrorResponse::new(format!("{:?}", e).as_str()))
            }
        }
    }
}


#[delete("/{id}")]
async fn delete(request: web::Path<models::ReceiptIdentifier>) -> impl Responder {
    match get_all_receipts() {
        None => HttpResponse::Ok().json(models::ErrorResponse::new("No receipts found")),
        Some(receipts) => {
            let request = request.into_inner();
            let total_receipts = receipts.len();
            let receipts: Vec<models::Receipt> = receipts
                .into_iter()
                .filter(|r| r.id != request.id)
                .collect();
            if total_receipts < receipts.len() {
                return HttpResponse::Ok().json(models::ErrorResponse::new(&format!("No receipt with id {} found", request.id)));
            }
            match write_receipt_file(&receipts) {
                Ok(_) => HttpResponse::Ok().json(receipts),
                Err(e) => HttpResponse::Ok().json(models::ErrorResponse::new(format!("{:?}", e).as_str()))
            }
        }
    }
}


#[get("/store/{store}")]
async fn get_by_store(info: web::Path<models::StoreIdentifier>) -> impl Responder {
    HttpResponse::Ok().json(format!("store: {}", info.store))
}


pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_all)
        .service(get_by_id)
        .service(get_by_store)
        .service(create)
        .service(delete);
    cfg.service(scope);
}