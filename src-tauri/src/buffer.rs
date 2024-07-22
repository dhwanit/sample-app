 let poppedup_tabs = browser.get_tabs().lock().unwrap();
    for tabbinger in poppedup_tabs.iter() {
        println!("Tabbinger URL: {}", tabbinger.get_url());
        if (tabbinger.get_url().contains("/simulate/popup")) {
            println!("Found the popup tab");
            tabbinger.wait_for_element("input#addfunds_amount")?;
            tabbinger
                .find_element("input#addfunds_amount")?
                .type_into("1000000")?;

            tabbinger.wait_for_element("button#addfunds_submit")?;
            tabbinger.find_element("button#addfunds_submit")?.click()?;

            tabbinger.wait_for_element("button#submit")?;
            tabbinger.find_element("button#submit")?.click()?;

            tabbinger.wait_for_element("button#submit")?;
            tabbinger.find_element("button#submit")?.click()?;
        }