use crate::error::Error;
use roxmltree::Document;
use std::collections::HashMap;

/// 解析外层加密的 XML，提取 Encrypt 字段
pub fn parse_encrypted_xml(xml: &str) -> Result<String, Error> {
    let doc = Document::parse(xml)?;
    let root = doc.root_element();

    // 查找 xml 标签下的 Encrypt 子标签
    for child in root.children() {
        if child.is_element() && child.tag_name().name() == "xml" {
            for xml_child in child.children() {
                if xml_child.is_element()
                    && xml_child.tag_name().name() == "Encrypt"
                    && let Some(text) = xml_child.text()
                {
                    return Ok(text.trim().to_string());
                }
            }
        }
        // 也可能直接在 root 下（没有外层 xml 标签）
        if child.is_element()
            && child.tag_name().name() == "Encrypt"
            && let Some(text) = child.text()
        {
            return Ok(text.trim().to_string());
        }
    }

    Err(Error::Config("Encrypt field not found in XML".to_string()))
}

/// 解析解密后的组件回调 XML
pub fn parse_component_xml(xml: &str) -> Result<ParsedComponentXml, Error> {
    let doc = Document::parse(xml)?;
    let root = doc.root_element();

    let mut app_id = None;
    let mut create_time = None;
    let mut info_type = None;
    let mut component_verify_ticket = None;

    // 查找 xml 标签
    for child in root.children() {
        if child.is_element() && child.tag_name().name() == "xml" {
            parse_component_xml_content(
                child,
                &mut app_id,
                &mut create_time,
                &mut info_type,
                &mut component_verify_ticket,
            );
        }
        // 也可能没有外层 xml 标签
        else if child.is_element() {
            match child.tag_name().name() {
                "AppId" => app_id = child.text().map(|s| s.trim().to_string()),
                "CreateTime" => {
                    if let Some(text) = child.text() {
                        create_time = text.trim().parse::<i64>().ok();
                    }
                }
                "InfoType" => info_type = child.text().map(|s| s.trim().to_string()),
                "ComponentVerifyTicket" => {
                    component_verify_ticket = child.text().map(|s| s.trim().to_string())
                }
                _ => {}
            }
        }
    }

    Ok(ParsedComponentXml {
        app_id: app_id.ok_or_else(|| Error::Config("AppId not found".to_string()))?,
        create_time: create_time
            .ok_or_else(|| Error::Config("CreateTime not found".to_string()))?,
        info_type: info_type.ok_or_else(|| Error::Config("InfoType not found".to_string()))?,
        component_verify_ticket,
        raw_xml: xml.to_string(),
    })
}

fn parse_component_xml_content(
    xml_node: roxmltree::Node,
    app_id: &mut Option<String>,
    create_time: &mut Option<i64>,
    info_type: &mut Option<String>,
    component_verify_ticket: &mut Option<String>,
) {
    for child in xml_node.children() {
        if !child.is_element() {
            continue;
        }
        match child.tag_name().name() {
            "AppId" => *app_id = child.text().map(|s| s.trim().to_string()),
            "CreateTime" => {
                if let Some(text) = child.text() {
                    *create_time = text.trim().parse::<i64>().ok();
                }
            }
            "InfoType" => *info_type = child.text().map(|s| s.trim().to_string()),
            "ComponentVerifyTicket" => {
                *component_verify_ticket = child.text().map(|s| s.trim().to_string())
            }
            _ => {}
        }
    }
}

/// 解析后的组件 XML 数据
#[derive(Debug, Clone)]
pub struct ParsedComponentXml {
    pub app_id: String,
    pub create_time: i64,
    pub info_type: String,
    pub component_verify_ticket: Option<String>,
    pub raw_xml: String,
}

/// 将 XML 解析为简单的键值对 HashMap
pub fn parse_xml_to_map(xml: &str) -> Result<HashMap<String, String>, Error> {
    let doc = Document::parse(xml)?;
    let mut map = HashMap::new();

    let root = doc.root_element();

    fn process_node(node: roxmltree::Node, map: &mut HashMap<String, String>, prefix: &str) {
        for child in node.children() {
            if !child.is_element() {
                continue;
            }
            let tag_name = child.tag_name().name();
            let full_name = if prefix.is_empty() {
                tag_name.to_string()
            } else {
                format!("{}.{}", prefix, tag_name)
            };

            if let Some(text) = child.text() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    map.insert(full_name.clone(), trimmed.to_string());
                }
            }

            // 递归处理子节点
            process_node(child, map, &full_name);
        }
    }

    process_node(root, &mut map, "");

    Ok(map)
}
