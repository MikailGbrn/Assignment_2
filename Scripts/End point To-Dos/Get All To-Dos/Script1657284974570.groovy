import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('To-Dos/GET All To-Dos'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, '[26].userId', '2')
WS.verifyElementPropertyValue(response, '[26].id', '27')
WS.verifyElementPropertyValue(response, '[26].title', 'veritatis pariatur delectus')
WS.verifyElementPropertyValue(response, '[26].completed', 'true')


WS.verifyElementPropertyValue(response, '[27].userId', '2')
WS.verifyElementPropertyValue(response, '[27].id', '28')
WS.verifyElementPropertyValue(response, '[27].title', 'nesciunt totam sit blanditiis sit')
WS.verifyElementPropertyValue(response, '[27].completed', 'false')

WS.verifyElementPropertyValue(response, '[29].userId', '2')
WS.verifyElementPropertyValue(response, '[29].id', '30')
WS.verifyElementPropertyValue(response, '[29].title', 'nemo perspiciatis repellat ut dolor libero commodi blanditiis omnis')
WS.verifyElementPropertyValue(response, '[29].completed', 'true')

WS.verifyElementPropertyValue(response, '[62].userId', '4')
WS.verifyElementPropertyValue(response, '[62].id', '63')
WS.verifyElementPropertyValue(response, '[62].title', 'doloremque aut dolores quidem fuga qui nulla')
WS.verifyElementPropertyValue(response, '[62].completed', 'true')

WS.verifyElementPropertyValue(response, '[63].userId', '4')
WS.verifyElementPropertyValue(response, '[63].id', '64')
WS.verifyElementPropertyValue(response, '[63].title', 'voluptas consequatur qui ut quia magnam nemo esse')
WS.verifyElementPropertyValue(response, '[63].completed', 'false')