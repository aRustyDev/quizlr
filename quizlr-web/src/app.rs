use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div style="max-width: 600px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;">
            <h1 style="text-align: center; color: #333;">"Quizlr"</h1>
            <p style="text-align: center; color: #666; margin-bottom: 30px;">"Adaptive Learning Platform"</p>

            <div style="background: #f9f9f9; padding: 20px; border-radius: 8px; margin-bottom: 20px; min-height: 300px;">
                <p style="color: #666; margin-bottom: 20px;">
                    "Question " <span id="question-num">"1"</span> " of 3"
                </p>

                <h3 style="margin-bottom: 20px;" id="question-text">"What is 2 + 2?"</h3>

                <div id="options-container">
                    <button
                        style="display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;"
                        onclick="handleAnswer(0, false)"
                    >
                        "3"
                    </button>

                    <button
                        style="display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;"
                        onclick="handleAnswer(1, true)"
                    >
                        "4"
                    </button>

                    <button
                        style="display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;"
                        onclick="handleAnswer(2, false)"
                    >
                        "5"
                    </button>

                    <button
                        style="display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;"
                        onclick="handleAnswer(3, false)"
                    >
                        "6"
                    </button>
                </div>

                <div id="next-button-container" style="display: none;">
                    <button
                        onclick="nextQuestion()"
                        style="display: block; width: 100%; padding: 15px; margin: 20px 0; border: none; border-radius: 4px; background: #2196F3; color: white; cursor: pointer; font-size: 16px;"
                    >
                        "Next Question"
                    </button>
                </div>

                <div id="quiz-complete" style="display: none;">
                    <h3 style="margin-bottom: 20px;">"Quiz Complete!"</h3>
                    <p style="font-size: 24px; margin: 20px 0; text-align: center;">
                        "Your score: " <span id="final-score">"0"</span> "/3"
                    </p>
                    <button
                        onclick="resetQuiz()"
                        style="display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: #4CAF50; color: white; cursor: pointer; font-size: 16px;"
                    >
                        "Start Again"
                    </button>
                </div>
            </div>

            <div style="text-align: center; color: #666;">
                "Score: " <span id="score">"0"</span> "/3"
            </div>
        </div>

        <script>
            "
            let currentQuestion = 0;
            let score = 0;
            
            const questions = [
                {
                    text: 'What is 2 + 2?',
                    options: ['3', '4', '5', '6'],
                    correctIndex: 1
                },
                {
                    text: 'What is the capital of France?',
                    options: ['London', 'Berlin', 'Paris', 'Madrid'],
                    correctIndex: 2
                },
                {
                    text: 'Which planet is closest to the Sun?',
                    options: ['Venus', 'Mercury', 'Earth', 'Mars'],
                    correctIndex: 1
                }
            ];
            
            function handleAnswer(optionIndex, isCorrect) {
                if (isCorrect) {
                    score++;
                    document.getElementById('score').textContent = score;
                }
                
                // Disable all buttons
                const buttons = document.querySelectorAll('#options-container button');
                buttons.forEach(btn => btn.disabled = true);
                
                // Show next button
                document.getElementById('next-button-container').style.display = 'block';
            }
            
            function nextQuestion() {
                currentQuestion++;
                
                if (currentQuestion >= questions.length) {
                    // Show quiz complete
                    document.getElementById('options-container').style.display = 'none';
                    document.getElementById('next-button-container').style.display = 'none';
                    document.getElementById('question-text').style.display = 'none';
                    document.getElementById('quiz-complete').style.display = 'block';
                    document.getElementById('final-score').textContent = score;
                } else {
                    // Load next question
                    const q = questions[currentQuestion];
                    document.getElementById('question-num').textContent = currentQuestion + 1;
                    document.getElementById('question-text').textContent = q.text;
                    
                    // Update options
                    const optionsHtml = q.options.map((opt, idx) => 
                        `<button style='display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;' 
                         onclick='handleAnswer(${idx}, ${idx === q.correctIndex})'>
                         ${opt}
                         </button>`
                    ).join('');
                    
                    document.getElementById('options-container').innerHTML = optionsHtml;
                    document.getElementById('next-button-container').style.display = 'none';
                }
            }
            
            function resetQuiz() {
                currentQuestion = 0;
                score = 0;
                document.getElementById('score').textContent = '0';
                document.getElementById('question-num').textContent = '1';
                document.getElementById('question-text').textContent = questions[0].text;
                document.getElementById('question-text').style.display = 'block';
                document.getElementById('quiz-complete').style.display = 'none';
                document.getElementById('options-container').style.display = 'block';
                
                // Reset to first question
                const q = questions[0];
                const optionsHtml = q.options.map((opt, idx) => 
                    `<button style='display: block; width: 100%; padding: 15px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 16px;' 
                     onclick='handleAnswer(${idx}, ${idx === q.correctIndex})'>
                     ${opt}
                     </button>`
                ).join('');
                
                document.getElementById('options-container').innerHTML = optionsHtml;
            }
            "
        </script>
    }
}
