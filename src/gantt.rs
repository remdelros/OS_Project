use crate::process::Process;

pub fn generate_gantt_chart(processes: &Vec<Process>) {
    // Sort processes by completion time for the visual representation
    let mut processes = processes.to_vec(); // Create a mutable copy
    processes.sort_by_key(|p| p.completion_time.unwrap_or(0));

    println!("----GANTT CHART----");
    generate_text_gantt_chart(processes);

    // Using plotlib for a graphical option (optional)
    if cfg!(feature = "plotlib") { // Use a Cargo feature if needed
        generate_plotlib_gantt_chart(processes);
    }
}


fn generate_text_gantt_chart(processes: &Vec<Process>) {
    let max_completion_time = processes.last().unwrap().completion_time.unwrap_or(0);
    let num_processes = processes.len();

    // Top Timeline
    print!("  ");
    for i in 0..=max_completion_time {
        print!("{:>2}", i); // Align numbers
    }
    println!("");

    // Process Blocks
    for (i, process) in processes.iter().enumerate() {
        print!("P{}|", process.pid); // Process label
        let start = process.arrival_time;
        let end = process.completion_time.unwrap_or(0);

        // Spaces before the process block
        for _ in 0..start {
            print!("  "); // Two spaces
        }

        // Process execution block
        for _ in 0..(end - start) {
            print!("██"); // Use a filled block character
        }
        println!("");

        // Separator line (optional)
        if i < num_processes - 1 {
            print!("  ");
            for _ in 0..=max_completion_time {
                print!("--");
            }
            println!("");
        }
    }

    #[cfg(feature = "plotlib")]
    fn generate_plotlib_gantt_chart(processes: &Vec<Process>) {
        let labels: Vec<String> = processes.iter().map(|p| format!("P{}", p.pid)).collect();
        let starts: Vec<f64> = processes.iter().map(|p| p.arrival_time as f64).collect();
        let durations: Vec<f64> = processes.iter().map(|p| p.burst_time as f64).collect();

        // Create a bar chart
        let data = BarChart::new_vec(&labels)
            .stacked()
            .bar_width(0.8)
            .set_style(vec![&PointStyle::new().marker(PointMarker::Square)]);

        // Create a continuous view to display the chart
        let v = ContinuousView::new()
            .add(data.clone().x_range(0.0, *starts.last().unwrap_or(&0.0) as f64 + 1.0))
            .add(data.x_range(0.0, *starts.last().unwrap_or(&0.0) as f64 + 1.0))
            .x_label("Time")
            .y_label("Processes");

        // Display the chart
        Page::single(&v).save("gantt.svg").unwrap();
    }
}
