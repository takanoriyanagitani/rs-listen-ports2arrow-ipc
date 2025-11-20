use std::error::Error;
use std::sync::Arc;

use arrow::array::{StringBuilder, UInt16Builder, UInt32Builder};
use arrow::ipc::writer::StreamWriter;
use arrow::record_batch::RecordBatch;
use rs_listen_ports2arrow_ipc::{ProtoInfo, get_listeners, listener_schema};

fn main() -> Result<(), Box<dyn Error>> {
    let schema = listener_schema();
    let listeners = get_listeners()?;

    let mut pid_builder = UInt32Builder::new();
    let mut process_name_builder = StringBuilder::new();
    let mut process_path_builder = StringBuilder::new();
    let mut local_ip_builder = StringBuilder::new();
    let mut local_port_builder = UInt16Builder::new();
    let mut protocol_builder = StringBuilder::new();
    let mut ip_version_builder = StringBuilder::new();

    for listener in listeners {
        let proto_info = ProtoInfo(listener.protocol);

        pid_builder.append_value(listener.process.pid);
        process_name_builder.append_value(&listener.process.name);
        process_path_builder.append_value(&listener.process.path);
        local_ip_builder.append_value(listener.socket.ip().to_string());
        local_port_builder.append_value(listener.socket.port());
        protocol_builder.append_value(proto_info.as_str());
        ip_version_builder.append_value(if listener.socket.is_ipv4() {
            "v4"
        } else {
            "v6"
        });
    }

    let pid_array = pid_builder.finish();
    let process_name_array = process_name_builder.finish();
    let process_path_array = process_path_builder.finish();
    let local_ip_array = local_ip_builder.finish();
    let local_port_array = local_port_builder.finish();
    let protocol_array = protocol_builder.finish();
    let ip_version_array = ip_version_builder.finish();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(pid_array),
            Arc::new(process_name_array),
            Arc::new(process_path_array),
            Arc::new(local_ip_array),
            Arc::new(local_port_array),
            Arc::new(protocol_array),
            Arc::new(ip_version_array),
        ],
    )?;

    let stdout = std::io::stdout();
    let mut writer = StreamWriter::try_new(stdout, &schema)?;
    writer.write(&batch)?;
    writer.finish()?;

    Ok(())
}
