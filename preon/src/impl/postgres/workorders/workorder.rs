use crate::models::workorder::{
    self,
    Entity as WorkOrderEntity,
    Model as WorkOrderModel,
    ActiveModel as WorkOrderActiveModel
};

use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{Result, Error};

pub struct AbstractWorkOrder;

impl AbstractWorkOrder {
    /// Create a new WorkOrder
    pub async fn create_work_order(
        db: &DbConn, 
        title: String,
        description: String,
        start_date: DateTimeWithTimeZone,
        end_date: Option<DateTimeWithTimeZone>,
        created_by: i32
    ) -> Result<WorkOrderModel> {
        let workorder = WorkOrderActiveModel {
            id: NotSet,
            title: Set(title),
            description: Set(description),
            status: Set(1),
            start_date: Set(start_date),
            end_date: Set(end_date),
            created_by: Set(created_by)
        };
        
        let workorder = workorder
            .insert(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "create_workorder", 
                with: "sessions",
                info: e.to_string()
            })?;
        
        Ok(workorder)
    }
    
    /// Paginate WorkOrders
    pub async fn workorder_pagination(db: &DbConn, user_id: i32, page: u64, workorder_per_page: u64) -> Result<(Vec<WorkOrderModel>, u64)> {
        let paginator = WorkOrderEntity::find()
            .filter(workorder::Column::CreatedBy.eq(user_id))
            .order_by_desc(workorder::Column::Id)
            .paginate(db, workorder_per_page);

        let num_pages = paginator
            .num_pages()
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "workorder_pagination", 
                with: "sessions",
                info: e.to_string()
            })?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, num_pages))
            .map_err(|e| Error::DatabaseError {
                operation: "workorder_pagination",
                with: "sessions",
                info: e.to_string(),
            })

    }
}