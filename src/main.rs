use yew::prelude::*;

struct Contacts {
    qso_number: i32,
    call_sign: String,
    frequency: f32,
    band: String,
    mode: String,
    qso_date: String,
    time_on: String,
    rst_sent: String,
    rst_rcvd: String,
    name: String,
    qth: String,
    qth_locator: String,
    tx_pwr: String,
    sota_ref: String,
    park_ref: String,
}

fn generate_table(contacts: Vec<Contacts>) -> Html {
    html! {
        <>
            <table style="border: 1px solid #000000;">
                <thead style="border: 1px solid #000000;">
                    <tr style="border: 1px solid #000000;">
                        <th>{ "QSO Number" }</th>
                        <th>{ "Call Sign" }</th>
                        <th>{ "Frequency" }</th>
                        <th>{ "Band" }</th>
                        <th>{ "Mode" }</th>
                        <th>{ "QSO Date" }</th>
                        <th>{ "Time On" }</th>
                        <th>{ "RST Sent" }</th>
                        <th>{ "RST Received" }</th>
                        <th>{ "Name" }</th>
                        <th>{ "QTH" }</th>
                        <th>{ "QTH Locator" }</th>
                        <th>{ "TX Power" }</th>
                        <th>{ "SOTA Reference" }</th>
                        <th>{ "PARK Reference" }</th>
                    </tr>
                </thead>
                <tbody style="border: 1px solid #000000;">
                    { generate_table_body(contacts) }
                </tbody>
            </table>
        </>
    }
}

fn generate_row_white(contact: &Contacts) -> Html {
    html! {
        <tr style="border: 1px solid #000000;">
            <td>{ contact.qso_number }</td>
            <td>{ contact.call_sign.clone() }</td>
            <td>{ contact.frequency }</td>
            <td>{ contact.band.clone() }</td>
            <td>{ contact.mode.clone() }</td>
            <td>{ contact.qso_date.clone() }</td>
            <td>{ contact.time_on.clone() }</td>
            <td>{ contact.rst_sent.clone() }</td>
            <td>{ contact.rst_rcvd.clone() }</td>
            <td>{ contact.name.clone() }</td>
            <td>{ contact.qth.clone() }</td>
            <td>{ contact.qth_locator.clone() }</td>
            <td>{ contact.tx_pwr.clone() }</td>
            <td>{ contact.sota_ref.clone() }</td>
            <td>{ contact.park_ref.clone() }</td>
        </tr>
    }
}

fn generate_row_light_grey(contact: &Contacts) -> Html {
    html! {
        <tr style="background-color: #f0f0f0;">
            <td>{ contact.qso_number }</td>
            <td>{ contact.call_sign.clone() }</td>
            <td>{ contact.frequency }</td>
            <td>{ contact.band.clone() }</td>
            <td>{ contact.mode.clone() }</td>
            <td>{ contact.qso_date.clone() }</td>
            <td>{ contact.time_on.clone() }</td>
            <td>{ contact.rst_sent.clone() }</td>
            <td>{ contact.rst_rcvd.clone() }</td>
            <td>{ contact.name.clone() }</td>
            <td>{ contact.qth.clone() }</td>
            <td>{ contact.qth_locator.clone() }</td>
            <td>{ contact.tx_pwr.clone() }</td>
            <td>{ contact.sota_ref.clone() }</td>
            <td>{ contact.park_ref.clone() }</td>
        </tr>
    }
}

fn generate_table_body(contacts: Vec<Contacts>) -> Html {
    // style="border: 1px solid #000000;" and alternate the row colors from white to light gray
    let mut index = 0;
    html! {
        <>
            { for contacts.iter().map(|contact| {
                index += 1;
                if index % 2 == 0 {
                    generate_row_light_grey(contact)
                } else {
                    generate_row_white(contact)
                }
            }) }
        </>
    }
}

fn generate_test_data() -> Vec<Contacts> {
    let mut contacts: Vec<Contacts> = Vec::new();
    for i in 0..10 {
        contacts.push(Contacts {
            qso_number: i,
            call_sign: "K0DAN".to_string(),
            frequency: 14.285,
            band: "20m".to_string(),
            mode: "SSB".to_string(),
            qso_date: "2021-07-04".to_string(),
            time_on: "00:00:00".to_string(),
            rst_sent: "59".to_string(),
            rst_rcvd: "59".to_string(),
            name: "Daniel".to_string(),
            qth: "Colorado".to_string(),
            qth_locator: "DM79".to_string(),
            tx_pwr: "100".to_string(),
            sota_ref: "W0C/FR-003".to_string(),
            park_ref: "K-0012".to_string(),
        });
    }
    contacts
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let contacts = generate_test_data();
    let table = generate_table(contacts);

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            { table }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
