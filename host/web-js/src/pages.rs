//! The pages rendered at build, and a type's zero: what the compiled module
//! tells the renderers and the runtime about a route before any answer.
use exact_plan::Plan;

/// The locations rendered at build (`render=build` routes without parameters,
/// and the not-found page), as JSON (LLP 1048.003 D5).
pub(super) fn build_pages(plan: &Plan) -> String {
    let rows: Vec<String> = plan
        .routes
        .iter()
        .filter(|r| {
            r.render == exact_plan::RenderPolicy::Build && !plan.str(r.pattern).contains(':')
        })
        .map(|r| {
            let location = if r.notfound {
                "/404"
            } else {
                plan.str(r.pattern)
            };
            format!(
                "{{\"location\":{},\"notfound\":{}}}",
                serde_json::to_string(location).unwrap(),
                r.notfound
            )
        })
        .collect();
    format!("[{}]", rows.join(","))
}

/// The type's zero, as the runner's `zero` makes it.
pub(super) fn zero(plan: &Plan, ty: exact_plan::TypesId) -> String {
    let t = &plan.types[ty.0 as usize];
    match t.kind {
        exact_plan::TypeKind::Number => "0".into(),
        exact_plan::TypeKind::Bool => "!1".into(),
        exact_plan::TypeKind::String => serde_json::to_string(
            t.fields
                .iter()
                .next()
                .map_or("", |f| plan.str(plan.field(f).name)),
        )
        .unwrap(),
        exact_plan::TypeKind::Unit | exact_plan::TypeKind::Option => "null".into(),
        exact_plan::TypeKind::List => "[]".into(),
        exact_plan::TypeKind::Record => format!(
            "[{}]",
            t.fields
                .iter()
                .map(|f| zero(plan, plan.fields[f.0 as usize].ty))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}
