// Box<T>はヒープ上にTを格納し、スマートポインタとして機能します。
// Option<T>は、値が存在しない（None）か存在する（Some(T)）かを安全に扱います。
// Node<T>の定義は、再帰的に自身を参照します（Boxがないとコンパイルエラーになります）。
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// 再帰的にノードを構築する関数。Option<Box<Node>>を返します。
fn create_node_recursive(depth: i32, max_depth: i32) -> Option<Box<Node>> {
    // 終了条件: 指定された深さに達したら、None（子ノードなし）を返します。
    if depth >= max_depth {
        return None;
    }

    // ノードの値を深さに応じて決定
    let current_value = depth * 10 + 1;

    // 左と右の子ノードを再帰的に生成
    let left_child = create_node_recursive(depth + 1, max_depth);
    let right_child = create_node_recursive(depth + 1, max_depth);

    // 新しいノードをヒープ上に構築し、Boxで包んで返します。
    Some(Box::new(Node {
        value: current_value,
        left: left_child,
        right: right_child,
    }))
}

// メイン関数
fn main() {
    let max_depth = 3;

    println!("🌲 深さ {} の二分木を構築します...", max_depth);

    // ルートノードを取得。ResultはOption<Box<Node>>
    let root_node = create_node_recursive(0, max_depth);

    match root_node {
        Some(node) => {
            println!("✅ 構築成功！ルートノードの値を表示:");
            println!("ルートノードの値: {}", node.value);
            // ノード全体をデバッグ出力（見やすい形式でツリー構造が出力されます）
            println!("\n🌳 木の構造:");
            // {:?} はDebugトレイトを使って構造体を整形して出力します
            println!("{:#?}", node); 
        }
        None => {
            println!("❌ 構築失敗。深さが0のため、ノードが作成されませんでした。");
        }
    }
}