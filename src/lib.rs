mod methods;
mod types;

use crate::types::Inventory;
use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::{
    raw, redis_command, redis_module, Context, NextArg, RedisError, RedisResult, RedisString,
    RedisValue, REDIS_OK,
};

static INVENTORY: RedisType = RedisType::new(
    "inventory",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(methods::rdb_load),
        rdb_save: Some(methods::rdb_save),
        aof_rewrite: None,
        free: Some(methods::free),

        // Currently unused by Redis
        mem_usage: Some(methods::mem_usage),
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);

fn create_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let total = args.next_u64()?.to_u32()?;

    ctx.log_debug(format!("new inventory: key = [{}], total = [{}]", key, total).as_str());

    let key = ctx.open_key_writable(&key);

    if let Some(value) = key.get_value::<Inventory>(&INVENTORY)? {
        value.reset(total);
    } else {
        let value = Inventory::new(total);
        key.set_value(&INVENTORY, value)?;
    }

    REDIS_OK
}

fn create_new_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name = args.next_arg()?;
    let total = args.next_u64()?.to_u32()?;

    ctx.log_debug(format!("new inventory: key = [{}], total = [{}]", key_name, total).as_str());

    let key = ctx.open_key_writable(&key_name);

    if key.get_value::<Inventory>(&INVENTORY)?.is_some() {
        Err(RedisError::String(format!(
            "Inventory '{}' exists",
            key_name
        )))
    } else {
        let value = Inventory::new(total);
        key.set_value(&INVENTORY, value)?;
        REDIS_OK
    }
}

fn deduct_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let deduction = args.next_u64().unwrap_or(1).to_u32()?;

    ctx.log_debug(
        format!(
            "deduct inventory: key = [{}], deduction = [{}]",
            key, deduction
        )
        .as_str(),
    );

    let key = ctx.open_key_writable(&key);

    if let Some(value) = key.get_value::<Inventory>(&INVENTORY)? {
        match value.take(deduction) {
            None => Err(RedisError::Str("Inventory shortage")),
            Some(current) => Ok(RedisValue::SimpleString(current.to_string())),
        }
    } else {
        Ok(RedisValue::Null)
    }
}

fn get_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;

    ctx.log_debug(format!("get inventory: key = [{}]", key).as_str());

    let key = ctx.open_key_writable(&key);

    get(&key)
}

fn increase_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let by = args.next_u64()?.to_u32()?;

    ctx.log_debug(format!("increase inventory: key = [{}], addition = [{}]", key, by).as_str());

    let key = ctx.open_key_writable(&key);

    if let Some(value) = key.get_value::<Inventory>(&INVENTORY)? {
        let (total, current) = value.increase(by);
        Ok(RedisValue::Array(vec![
            RedisValue::SimpleString(total.to_string()),
            RedisValue::SimpleString(current.to_string()),
        ]))
    } else {
        Ok(RedisValue::Null)
    }
}

fn return_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let return_ = args.next_u64()?.to_u32()?;

    ctx.log_debug(format!("return inventory: key = [{}], return = [{}]", key, return_).as_str());

    let key = ctx.open_key_writable(&key);

    if let Some(value) = key.get_value::<Inventory>(&INVENTORY)? {
        match value.r#return(return_) {
            None => Err(RedisError::Str("Remaining inventory exceeding total")),
            Some(current) => Ok(RedisValue::SimpleString(current.to_string())),
        }
    } else {
        Ok(RedisValue::Null)
    }
}

fn delete_inventory(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;

    ctx.log_debug(format!("delete inventory: key = [{}]", key).as_str());

    let key = ctx.open_key_writable(&key);

    let result = get(&key);

    let _ = key.delete()?;
    result
}

fn get(key: &RedisKeyWritable) -> RedisResult {
    if let Some(value) = key.get_value::<Inventory>(&INVENTORY)? {
        Ok(RedisValue::Array(vec![
            RedisValue::SimpleString(value.total().to_string()),
            RedisValue::SimpleString(value.current().to_string()),
        ]))
    } else {
        Ok(RedisValue::Null)
    }
}

trait ToU32 {
    fn to_u32(self) -> Result<u32, RedisError>;
}

impl ToU32 for u64 {
    fn to_u32(self) -> Result<u32, RedisError> {
        if self > u32::MAX as u64 {
            Err(RedisError::Str("out of range, max = 4294967295"))
        } else {
            Ok(self as u32)
        }
    }
}

redis_module! {
    name: "inventory",
    version: 1,
    data_types: [
        INVENTORY,
    ],
    commands: [
        ["inv.set", create_inventory, "write deny-oom", 1, 1, 1],
        ["inv.setnx", create_new_inventory, "write deny-oom", 1, 1, 1],
        ["inv.get", get_inventory, "readonly", 1, 1, 1],
        ["inv.ddct", deduct_inventory, "write", 1, 1, 1],
        ["inv.incr", increase_inventory, "write", 1, 1, 1],
        ["inv.return", return_inventory, "write", 1, 1, 1],
        ["inv.del", delete_inventory, "write", 1, 1, 1],
    ],
}
